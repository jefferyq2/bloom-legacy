package billing

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func ChangePlan(ctx context.Context, actor *users.User, userId, groupId *string, planId string) error {
	logger := rz.FromCtx(ctx)
	var customer *Customer
	var err error

	// validate params
	if actor == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return NewError(ErrorChangingPlan)
	}

	if userId != nil && groupId != nil {
		return NewError(ErrorUserIdAndGroupIdCantBeBothNonNull)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.ChangePlan: Starting transaction", rz.Err(err))
		return NewError(ErrorChangingPlan)
	}

	if userId != nil {
		if *userId != actor.ID && !actor.IsAdmin {
			tx.Rollback()
			return NewError(ErrorAdminRoleRequired)
		}
		customer, err = FindCustomerByUserId(ctx, tx, *userId)
		if err != nil {
			tx.Rollback()
			return err
		}
	} else { // groupId != nil
		if !actor.IsAdmin {
			if err = groups.CheckUserIsGroupAdmin(ctx, tx, actor.ID, *groupId); err != nil {
				tx.Rollback()
				return err
			}
		}
		customer, err = FindCustomerByGroupId(ctx, tx, *groupId)
		if err != nil {
			tx.Rollback()
			return err
		}
	}

	newPlan, err := FindPlanActiveById(ctx, tx, planId)
	if err != nil {
		tx.Rollback()
		logger.Warn("billing.ChangePlan:f inding newPlan", rz.Err(err), rz.String("id", planId))
		return err
	}

	oldPlan, err := FindPlanActiveById(ctx, tx, customer.PlanID)
	if err != nil {
		tx.Rollback()
		logger.Warn("billing.ChangePlan: finding old plan", rz.Err(err), rz.String("id", customer.PlanID))
		return err
	}

	// check the ability to change plan (used storage)
	if newPlan.ID == oldPlan.ID {
		tx.Rollback()
		return NewError(ErrorOldPlanIsTheSameAsNewPlan)
	}
	newAllowedStorage := GetAllowedStorageFromPlanTier(newPlan.Tier)
	if customer.UsedStorage > newAllowedStorage {
		tx.Rollback()
		return NewError(ErrorTooMuchStorageUsedForNewPlan)
	}
	// update customer
	queryUpdate := "UPDATE billing_customers SET updated_at = $1, plan_id = $2 WHERE id = $3"

	// update oldDefaultPaymentMethod
	customer.UpdatedAt = time.Now().UTC()
	customer.PlanID = newPlan.ID
	_, err = tx.Exec(queryUpdate, customer.UpdatedAt, customer.PlanID, customer.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangePlan: customer", rz.Err(err))
		return NewError(ErrorChangingPlan)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangePlan: Committing transaction", rz.Err(err))
		return NewError(ErrorChangingPlan)
	}

	return nil
}

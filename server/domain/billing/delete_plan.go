package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeletePlan(ctx context.Context, tx *sqlx.Tx, user *users.User, planId string) error {
	var err error
	logger := rz.FromCtx(ctx)

	if !user.IsAdmin {
		return NewError(ErrorAdminRolRequired)
	}

	// delete plan
	queryDeleteGroup := "DELETE FROM billing_plans WHERE id = $1"
	_, err = tx.Exec(queryDeleteGroup, planId)
	if err != nil {
		logger.Error("billing.DeletePlan: deleting plan", rz.Err(err))
		return NewError(ErrorDeletingPlan)
	}

	return err
}

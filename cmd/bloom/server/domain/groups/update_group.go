package groups

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

type UpdateGroupParams struct {
	ID          uuid.UUID
	Name        *string
	Description *string
}

func UpdateGroup(ctx context.Context, actor *users.User, params UpdateGroupParams) (ret *Group, err error) {
	logger := rz.FromCtx(ctx)
	var newName string
	var newDescription string

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.UpdateGroup: Starting transaction", rz.Err(err))
		err = NewError(ErrorUpdatingGroup)
		return
	}

	ret, err = FindGroupById(ctx, tx, params.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.UpdateGroup: fetching group", rz.Err(err),
			rz.String("group.id", params.ID.String()))
		err = NewError(ErrorGroupNotFound)
		return
	}

	if params.Name == nil {
		newName = ret.Name
	} else {
		newName = *params.Name
	}

	if params.Description == nil {
		newDescription = ret.Description
	} else {
		newDescription = *params.Description
	}

	err = validateUpdateGroup(ctx, tx, actor.ID, ret.ID, newName, newDescription)
	if err != nil {
		tx.Rollback()
		return
	}

	ret.UpdatedAt = time.Now().UTC()
	ret.Name = newName
	ret.Description = newDescription
	queryUpdateGroup := `UPDATE groups
		SET updated_at = $1, name = $2, description = $3
		WHERE id = $4`
	_, err = tx.Exec(queryUpdateGroup, ret.UpdatedAt, ret.Name, ret.Description, ret.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.UpdateGroup: updating group", rz.Err(err))
		err = NewError(ErrorUpdatingGroup)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.UpdateGroup: Committing transaction", rz.Err(err))
		err = NewError(ErrorUpdatingGroup)
	}

	return
}

// validateUpdateGroup Checks that user is member of group and he has administrator role
func validateUpdateGroup(ctx context.Context, tx *sqlx.Tx, userID, groupID uuid.UUID, name, description string) error {
	var err error

	if err = CheckUserIsGroupAdmin(ctx, tx, userID, groupID); err != nil {
		return err
	}

	if err = validator.GroupName(name); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if err = validator.GroupDescription(description); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	return nil
}

package mutation

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/rz"
)

func (resolver *Resolver) SendNewRegistrationCode(ctx context.Context, input model.SendNewRegistrationCodeInput) (ret bool, err error) {
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser != nil {
		return ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(500, 800)
	if err != nil {
		logger.Error("mutaiton.SendNewRegistrationCode: generating random int", rz.Err(err))
		err = gqlerrors.Internal()
		return
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	err = users.SendNewRegistrationCode(ctx, input.ID)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = true
	return
}
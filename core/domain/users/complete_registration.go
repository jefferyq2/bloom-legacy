package users

import (
	"context"
	"errors"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/coreutil"
	"gitlab.com/bloom42/libs/graphql-go"
)

func CompleteRegistration(params CompleteRegistrationParams) (model.SignedIn, error) {
	client := api.Client()
	var ret model.SignedIn

	authKey := deriveAuthKey([]byte(params.Username), []byte(params.Password))
	if authKey == nil {
		return ret, errors.New("Error deriving auth key")
	}

	input := model.CompleteRegistrationInput{
		ID:       params.ID,
		Username: params.Username,
		AuthKey:  authKey,
		Device: &model.SessionDeviceInput{
			Os:   model.SessionDeviceOs(coreutil.GetDeviceOS()),
			Type: model.SessionDeviceType(coreutil.GetDeviceType()),
		},
	}
	var resp struct {
		CompleteRegistration *model.SignedIn `json:"completeRegistration"`
	}
	req := graphql.NewRequest(`
        mutation ($input: CompleteRegistrationInput!) {
			completeRegistration (input: $input) {
				session {
					id
					token
				}
				me {
					username
					displayName
					isAdmin
					avatarUrl
				}
			}
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)
	if resp.CompleteRegistration != nil {
		ret = *resp.CompleteRegistration
		if ret.Session != nil && ret.Session.Token != nil {
			client.Authenticate(ret.Session.ID, *ret.Session.Token)
		}
	}

	return ret, err
}

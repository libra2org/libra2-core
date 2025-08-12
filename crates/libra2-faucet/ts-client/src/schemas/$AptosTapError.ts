/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export const $Libra2TapError = {
    description: `This is the generic struct we use for all API errors, it contains a string
    message and a service specific error code.`,
    properties: {
        message: {
            type: 'string',
            description: `A message describing the error`,
            isRequired: true,
        },
        error_code: {
            type: 'all-of',
            contains: [{
                type: 'Libra2TapErrorCode',
            }],
            isRequired: true,
        },
        rejection_reasons: {
            type: 'array',
            contains: {
                type: 'RejectionReason',
            },
            isRequired: true,
        },
        txn_hashes: {
            type: 'array',
            contains: {
                type: 'string',
            },
            isRequired: true,
        },
    },
} as const;

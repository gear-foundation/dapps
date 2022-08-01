export const payloads = {
    init: function(
        fee_to_setter: string,
    ) {
        return {
           fee_to_setter,
        }
    },
    set_fee_to: function(
        fee_to: string,
    ) {
        return {
            SetFeeTo: {
                fee_to,
            }
        }
    },
    set_fee_to_setter: function(
        fee_to_setter: string,
    ) {
        return {
            SetFeeToSetter: {
                fee_to_setter,
            }
        }
    },
    create_pair: function(
        token_a: string,
        token_b: string,
    ) {
        return {
            CreatePair: {
                token_a,
                token_b,
            },
        }
    }
};
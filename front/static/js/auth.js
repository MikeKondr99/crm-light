(() => {
    
    function auth() {
        this.form = $('[name=auth]');
    }

    const _proto = auth.prototype;

    const EMAIL_REGEXP = /^(([^<>()[\].,;:\s@"]+(\.[^<>()[\].,;:\s@"]+)*)|(".+"))@(([^<>()[\].,;:\s@"]+\.)+[^<>()[\].,;:\s@"]{2,})$/iu;

    function onInput() {
        if (isEmailValid(input.value)) {
            input.style.borderColor = 'green';
        } else {
            input.style.borderColor = 'red';
        }
    }

    _proto.init = function() {
        const _this = this;

        _this.form.submit((e) => {
            e.preventDefault();
            _this.get();
        });
    }

    _proto.get = function() {
        const _this = this;
        const params = _this.form.serialize();

        // тут запрос к бэку
        
        _this.form.find('[name=auth-error]').append('ошибка');

        console.log(params)
    }

    return new auth;
})().init();

document.addEventListener('DOMContentLoaded', function() {
    var forms = document.querySelectorAll('form');
    forms.forEach(function(form) {
        form.addEventListener('submit', function(e) {
            var requiredFields = form.querySelectorAll('[required]');
            var valid = true;
            requiredFields.forEach(function(field) {
                if (!field.value.trim()) {
                    valid = false;
                    field.style.borderColor = '#dc3545';
                } else {
                    field.style.borderColor = '#ddd';
                }
            });
            if (!valid) {
                e.preventDefault();
            }
        });
    });

    var inputs = document.querySelectorAll('input, select, textarea');
    inputs.forEach(function(input) {
        input.addEventListener('focus', function() {
            this.style.borderColor = '#4a7c28';
        });
        input.addEventListener('blur', function() {
            this.style.borderColor = '#ddd';
        });
    });
});

angular
    .module('clog', ['hc.marked'])
    .controller('AppController', function ($http) {
        var self = this;
        var API_URL = '/generate'
        self.clogData = { repository: '' };

        this.submitForm = function (data) {
            self.isLoading = true;
            self.result = {};
            $http({
                method: 'POST',
                url: API_URL,
                data: data
            })
            .then(function (response) {
                self.isLoading = false;
                self.result = response.data;
            });
        };
    });

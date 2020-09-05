import Vue from 'vue';
import axios from 'axios';

var app = new Vue({
	el: '#app',
	data: {
		message: "Hello World",
		count: 0,
		user_id: "<not registered yet>"
	},
	methods: {
		poke: function() {
			var that = this;
			var user = document.getElementById('other_user').value;
			axios.get("http://localhost:8000/notify?uuid=" + user).then(
				function(response) {
					if (response.status == 200) {
						that.message = "Successfully requested to poke " + user + ". Answer was " + response.data;
					}
					else {
						that.message = "Failed to poke " + user;
					}
				}
			)
		}
	}
})

var promise = axios.get("http://localhost:8000/login?name=hello");
promise.then(function (response) {
    console.log(response.data);
    app.user_id = response.data;
  });

function loop() {
	axios.get("http://localhost:8000/poll?uuid="+app.user_id+"&seconds=10").then(
		function(response) {
			if (response.data == "Done") {
				app.count+=1;
			}
			loop();
		}
	);
}
loop();

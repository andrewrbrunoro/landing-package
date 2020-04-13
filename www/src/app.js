export default class App {

    constructor() {
        this.states = document.getElementById('state');

        this.setup();
    }

    setup () {
        this.states.addEventListener('change', () => {
            console.log(this);
        });
    }
}

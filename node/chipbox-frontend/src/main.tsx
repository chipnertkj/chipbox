import { render, scene } from "chipbox-solid-render";

function App() {
    return (
        <box width="100su" />
    );
}

render(() => <App />, scene.root());

import { render } from "chipbox-solid-render";

function App() {
    return <grid array_limit={3} />;
}

render(() => <App />, <box width="100su" height="100su" />);

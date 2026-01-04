import { render } from "chipbox-solid-render";
import { createSignal } from "solid-js";
import { info } from "chipbox:tracing";

info(`test`);

function App() {
    return <grid array_limit={3} />;
}

const result = App();
info(`App returned: ${result}`);
render(() => <App />, <box width="100su" height="100su" />);

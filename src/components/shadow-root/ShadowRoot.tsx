export default function ShadowRoot({child}: { child: JSX.Element }) {

    const attachShadow = (host: any) => {
        if (host == null) {
            return;
        }
        host.attachShadow({mode: "open"});
        host.shadowRoot.innerHTML = host.innerHTML;
        host.innerHTML = "";

        const children: Element[] = Array.from(host.shadowRoot.children);
        if (children.length === 0 || children.length > 1) {
            // TODO handle this case in Snacks
            console.error("Expected exactly one child in shadow root");
        }

        children[0].querySelectorAll("a").forEach((anchor) => {
            anchor.onclick = (event) => {
                event.preventDefault();
                // TODO open from rust
            }
        });
    }


    return <span ref={attachShadow}>{child}</span>;
}

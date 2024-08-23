import React from "react";

export class ShadowRoot extends React.Component<{ child: JSX.Element }> {

    attachShadow(host: any) {
        if (host == null) {
            return;
        }
        host.attachShadow({mode: "open"});
        host.shadowRoot.innerHTML = host.innerHTML;
        host.innerHTML = "";
    }

    render() {
        return (
            <span ref={this.attachShadow}>
                {this.props.child}
            </span>
        );
    }

}

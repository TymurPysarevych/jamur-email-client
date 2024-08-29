import {useRecoilValue} from "recoil";
import {loadingState} from "../../state/atoms.ts";
import {DNA} from "react-loader-spinner";

export default function LoadingComponent() {
    const loading = useRecoilValue<boolean>(loadingState);
    return (
        <div style={{position: 'fixed', top: '0', right: '0'}}>
            <DNA
                visible={loading}
                height="60"
                width="80"
                ariaLabel="dna-loading"
                wrapperStyle={{}}
                wrapperClass="dna-wrapper"
            />
        </div>
    )
}

export default function RenderIf(props) {
    return props.condition ? props.children : null;
}
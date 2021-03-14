import { useState } from "react";

import RenderIf from "./RenderIf";

import './App.css';

export default function App() {
    const [data, setData] = useState([]);

    const onFetchData = () => {
        fetch("/api/all")
            .then(response => {
                console.log(response.status);
            })
            .catch(error => console.error(error));
    };

    return (
        <>
            <h1>Learning Actix.rs</h1>
            <input type="button" value="fetch data" onClick={onFetchData} />

            <RenderIf condition={data.length === 0}>
                <h3>No data available</h3>
            </RenderIf>

            <RenderIf condition={data.length > 0}>
                <li>
                    {data.map((entry, index) => {
                        return (
                            <ul key={index}>{entry}</ul>
                        );
                    })}
                </li>
            </RenderIf>
        </>
    );
}

import React, { useEffect, useRef } from "react";
import { StoryCard, StoryCardEntry } from "./battle";

function StoryCardEntry(props: { entry: StoryCardEntry }): React.ReactNode {
    const { entry } = props;
    if ("h1" in entry) {
        return <h1>{entry.h1}</h1>;
    } else if ("p" in entry) {
        return <p>{entry.p}</p>;
    }
}

export function StoryCard(props: {
    storyCard: StoryCard;
    show: boolean;
    onClose: () => void;
}): React.ReactNode {
    const buttonRef = useRef<HTMLDialogElement>(null);
    useEffect(() => {
        if (buttonRef.current) {
            if (props.show) {
                buttonRef.current.showModal();
            } else {
                buttonRef.current.close();
            }
        }
    }, [props.show, buttonRef]);

    return (
        <dialog ref={buttonRef}>
            <button onClick={props.onClose}>X</button>
            {props.storyCard.map((entry, index) => (
                <StoryCardEntry key={index} entry={entry} />
            ))}
        </dialog>
    );
}

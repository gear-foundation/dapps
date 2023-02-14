interface Props {
    id: string;
    xoffset: number;
    backgroundColor: string;
    eventEmoji?: null | string;
}

export const RocketRace = (props: Props) => {
    const { xoffset = 0, backgroundColor } = props;

    return (
        <div className="h-1/4 w-full border-2"
             style={{background: backgroundColor}}>
                <span className="text-h2" style={{
                    position: "absolute",
                    left: `${xoffset}%`,
                }}>{"🚀"}</span>
        </div>
    )
}

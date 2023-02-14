import {RaceStatus} from "../../../pages/launch";

interface Props {
    id: string;
    xoffset: number;
    backgroundColor: string;
    eventEmoji?: null | string;
    status: RaceStatus,
}

export const RocketRace = (props: Props) => {
    const { xoffset = 0, backgroundColor = '#ADB2AF', status } = props;

    return (
        <div className="h-1/4 w-full border-2 border-b-gray-900"
             style={{background: status === RaceStatus.Registration ? '#ADB2AF' : backgroundColor}}>
                <span className="text-h2" style={{
                    position: "absolute",
                    left: `${xoffset}%`,
                }}>{"🚀"}</span>
        </div>
    )
}

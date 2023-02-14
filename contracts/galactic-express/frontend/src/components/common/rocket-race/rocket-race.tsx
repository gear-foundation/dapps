import {RacePosition} from "../../../pages/launch";

interface Props  extends RacePosition {}

export const RocketRace = (props: Props) => {
    const { xoffset = 0, bgColor = '#ADB2AF', id } = props;

    return (
        <div className="h-1/4 w-full border-2 border-b-gray-900"
             key={id}
             style={{background: bgColor}}>
                <span className="text-h2" style={{
                    position: "absolute",
                    left: `${xoffset}%`,
                }}>{`🚀 ${id.slice(1, 5)}...${id.slice(-5)}`}</span>
        </div>
    )
}

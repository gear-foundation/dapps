import {RacePosition} from "../../../pages/launch";
import {SessionStatus} from "../../../app/types/battles";

interface Props  extends RacePosition {
    sessionStatus: SessionStatus
}

export const RocketRace = (props: Props) => {
    const { xoffset = 0, bgColor = '#ADB2AF', id, sessionStatus } = props;

    function handleAddressBySessionStatus(sessionStatus: SessionStatus): string {
        if(sessionStatus === SessionStatus.REGISTRATION) {
            return `${id.slice(1, 5)}...${id.slice(-5)}`
        }

        return ''
    }

    return (
        <div className="h-1/4 w-full border-2 border-b-gray-900"
             key={id}
             style={{background: bgColor}}>
                <span className="text-h2" style={{
                    position: "absolute",
                    left: `${xoffset}%`,
                }}>{`🚀${handleAddressBySessionStatus(sessionStatus)}`}</span>
        </div>
    )
}

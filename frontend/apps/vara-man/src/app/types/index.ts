import { ReactNode } from 'react'

declare global {
	type BaseComponentProps = {
		children?: ReactNode
		className?: string
	}
	interface CanvasRenderingContext2D {
		imageSmoothingEnabled: boolean
		mozImageSmoothingEnabled: boolean
		webkitImageSmoothingEnabled: boolean
	}
}

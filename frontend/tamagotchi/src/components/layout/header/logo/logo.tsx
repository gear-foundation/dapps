import { Link, useLocation } from 'react-router-dom'
import { SpriteIcon } from '@/components/ui/sprite-icon'

export const Logo = () => {
  const { pathname } = useLocation()

  return (
    <>
      {pathname !== '/' ? (
        <Link
          to="/"
          className="inline-flex text-white transition-colors hover:text-opacity-70"
        >
          <SpriteIcon name="logo" width={180} height={44} className="h-10" />
        </Link>
      ) : (
        <span className="inline-flex text-white">
          <SpriteIcon name="logo" width={180} height={44} className="h-10" />
        </span>
      )}
    </>
  )
}

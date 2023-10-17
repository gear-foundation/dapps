import { Link } from 'react-router-dom'
import { cn } from '@/app/utils'
import { buttonStyles } from '@gear-js/ui'
import { useFTStore, useLessons } from '@/app/context'
import { StoreItemCard } from '@/components/cards/store-item-card'
import { SpriteIcon } from '@/components/ui/sprite-icon'

export default function Store() {
  const { lesson } = useLessons()
  const { items } = useFTStore()

  return (
    <>
      <h1 className="text-2xl font-kanit font-bold">Store</h1>
      {lesson?.programId ? (
        items.length ? (
          <ul className="mt-8 mb-10 grid grid-cols-3 gap-8">
            {items.map((item, i) => (
              <li key={i}>
                <StoreItemCard item={item} />
              </li>
            ))}
          </ul>
        ) : (
          <p className="my-auto opacity-70 text-center">Items not found</p>
        )
      ) : (
        <p className="my-auto opacity-70 text-center">
          Please, connect your Tamagotchi
        </p>
      )}
      <div className="mt-auto">
        <Link
          to="/"
          className={cn('btn gap-2 whitespace-nowrap', buttonStyles.light)}
        >
          <SpriteIcon name="left-arrow" className="w-5 h-5" />
          Back
        </Link>
      </div>
    </>
  )
}

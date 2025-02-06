import { Menu, Transition } from '@headlessui/react';
import { Fragment } from 'react';

import { useApp, useLessons, useTamagotchi } from '@/app/context';
import { cn } from '@/app/utils';
import { SpriteIcon } from '@/components/ui/sprite-icon';

export const AccountActionsMenu = () => {
  const { isPending } = useApp();
  const { setTamagotchi } = useTamagotchi();
  const { resetLesson } = useLessons();

  const initialOptions = [
    {
      id: 4,
      label: 'Upload Contract',
      action: () => {
        setTamagotchi(undefined);
        resetLesson();
      },
      icon: 'upload',
    },
  ];

  return (
    <div className="">
      <Menu as="div" className="relative inline-block">
        {({ open }) => (
          <>
            <Menu.Button
              className={cn(
                'inline-flex w-full justify-center rounded-full bg-white px-4 py-1.5 text-sm font-semibold font-kanit text-white transition-colors',
                'focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75',
                open ? 'bg-opacity-30' : 'bg-opacity-10 hover:bg-opacity-30',
                isPending && 'opacity-50 pointer-events-none',
              )}
              disabled={isPending}>
              Options
            </Menu.Button>
            <Transition
              as={Fragment}
              enter="transition ease-out duration-100"
              enterFrom="transform opacity-0 scale-95"
              enterTo="transform opacity-100 scale-100"
              leave="transition ease-in duration-75"
              leaveFrom="transform opacity-100 scale-100"
              leaveTo="transform opacity-0 scale-95">
              <Menu.Items className="absolute right-0 mt-2 origin-top-right divide-y divide-gray-100 rounded-md bg-[#353535] shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                <div className="py-2 font-kanit font-semibold text-sm whitespace-nowrap">
                  {initialOptions.map((item) => (
                    <Menu.Item key={item.id}>
                      {({ active }) => (
                        <button
                          className={cn(
                            'flex items-center gap-2 w-full px-6 py-2 text-white transition-colors',
                            active && 'text-opacity-70',
                          )}
                          onClick={item.action}>
                          <SpriteIcon name={item.icon} className="w-5 h-5" />
                          {item.label}
                        </button>
                      )}
                    </Menu.Item>
                  ))}
                </div>
              </Menu.Items>
            </Transition>
          </>
        )}
      </Menu>
    </div>
  );
};

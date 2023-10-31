import { useEffect, useState } from 'react'
import { ADDRESS } from '@/app/consts'
import { useAlert } from '@gear-js/react-hooks'
import { atom, useAtomValue, useSetAtom } from 'jotai'

const get = <T>(url: string) =>
  fetch(url, {
    method: 'GET',
  }).then(async (res) => {
    const json = await res.json()
    return json as T
  })

type INode = {
  address: string
  isCustom: boolean
  icon?: string
}

type INodeSection = {
  caption: string
  nodes: INode[]
}

type ICustomNode = INode & {
  caption: string
}

const nodesAtom = atom<ICustomNode[] | undefined>(undefined)

export function useNodes() {
  const nodes = useAtomValue(nodesAtom)
  const setNodes = useSetAtom(nodesAtom)

  return { nodes, setNodes }
}

export function useNodesSync() {
  const alert = useAlert()
  const { setNodes } = useNodes()
  const [, setLoading] = useState(false)

  useEffect(() => {
    setLoading(true)

    const getNodes = async () => {
      try {
        const res1 = await get<INodeSection[]>(ADDRESS.BASE_NODES)
        const res2 = await get<INodeSection[]>(ADDRESS.STAGING_NODES)
        const merged = [...res1, ...res2]
          .map((n) => n.nodes.map((node) => ({ ...node, caption: n.caption })))
          .flat()

        const nodes = [...new Map(merged.map((o) => [o.address, o])).values()]

        setNodes(nodes)
        // console.log({ nodes })
      } catch (e) {
        alert.error((e as any).message)
      } finally {
        setLoading(false)
      }
    }
    getNodes()

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])
}

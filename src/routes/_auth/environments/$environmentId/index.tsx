import { createFileRoute, redirect } from '@tanstack/react-router'

export const Route = createFileRoute('/_auth/environments/$environmentId/')({
  beforeLoad: ({ params }) =>
    redirect({
      to: `/environments/$environmentId/apps`,
      params: { environmentId: params.environmentId },
    }),
})

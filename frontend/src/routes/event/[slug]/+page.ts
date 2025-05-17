import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
    const { slug } = params;
    return {
        event: {
        title: `Event ${slug}`,
        description: `Description for event ${slug}`,
        date: new Date().toISOString(),
        },
    };
}
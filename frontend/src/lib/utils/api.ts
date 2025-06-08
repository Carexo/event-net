import {PUBLIC_API_URL} from '$env/static/public';

export const getApiUrl = (path: string): string => {
    const url = PUBLIC_API_URL || 'http://localhost:3000';

    if (path.startsWith('/')) {
        return `${url}${path}`;
    }

    return `${url}/${path}`;
}
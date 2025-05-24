import { API_URL } from '$env/static/private';

export const getApiUrl = (path: string): string => {
    const url = API_URL || 'http://localhost:3000';

    if (path.startsWith('/')) {
        return `${API_URL}${path}`;
    }

    return `${API_URL}/${path}`;
}
export interface PaginationParams {
    page: number;
    limit: number;
}

export interface PaginatedResponse<T> {
    items: T[];
    total: number;
    page: number;
    limit: number;
    pages: number;
}

export interface ApiResponse<T> {
    data: T;
    message: string;
}
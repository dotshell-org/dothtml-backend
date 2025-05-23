import { Request, Response, NextFunction } from 'express';

const excludePaths = ['/contact'];

export function apiKeyAuthMiddleware(): (req: Request, res: Response, next: NextFunction) => void {
  return (req: Request, res: Response, next: NextFunction) => {
    if (excludePaths.includes(req.path)) {
      return next();
    }
    const apiKey = req.header('x-api-key') || req.query.api_key;
    if (!apiKey || apiKey !== process.env.API_KEY) {
      return res.status(401).json({ error: 'Unauthorized: invalid or missing API key.' });
    }
    next();
  };
}

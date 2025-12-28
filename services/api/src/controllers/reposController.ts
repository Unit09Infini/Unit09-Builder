import type { Request, Response } from "express";
import type { Unit09Service } from "./types";
import { requireString, parsePagination } from "../utils/validators";

export interface ReposControllerDeps {
  unit09Service: Unit09Service;
}

export function createReposController(deps: ReposControllerDeps) {
  const { unit09Service } = deps;

  return {
    listRepos: async (req: Request, res: Response) => {
      const { limit, offset } = parsePagination(req.query);
      const repos = await unit09Service.listRepos();

      const slice = repos.slice(offset, offset + limit);
      res.json({
        items: slice,
        total: repos.length,
        limit,
        offset
      });
    },
    listModulesByRepo: async (req: Request, res: Response) => {
      const repoKey = requireString(req.params.repoKey, "repoKey");
      const modules = await unit09Service.listModulesByRepo(repoKey);
      res.json({ items: modules });
    }
  };
}

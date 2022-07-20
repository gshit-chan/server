/*
  Warnings:

  - You are about to drop the column `userHash` on the `Comment` table. All the data in the column will be lost.
  - You are about to drop the column `userHash` on the `Thread` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Comment" DROP COLUMN "userHash";

-- AlterTable
ALTER TABLE "Thread" DROP COLUMN "userHash";

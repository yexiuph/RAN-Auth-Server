USE [RanUser]
GO
/****** Object:  StoredProcedure [dbo].[GSAuth]    Script Date: 1/8/2024 5:57:39 PM ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
ALTER PROCEDURE [dbo].[GSAuth] (
    @UserID VARCHAR(20),
    @UserPass VARCHAR(50)
)
AS
BEGIN
    SET NOCOUNT ON;

    DECLARE @UserName VARCHAR(20);
    DECLARE @StoredPassword VARCHAR(50);
    DECLARE @UserAvailable INT;
    DECLARE @AuthenticationStatus BIT;
    DECLARE @UserNum INT;

    BEGIN TRY
        BEGIN TRAN; -- Start the transaction

        -- Check if the user exists and is available
        SELECT @UserNum = UserNum, @UserName = UserID, @StoredPassword = UserPass, @UserAvailable = UserAvailable
        FROM GSUserInfo
        WHERE UserID = @UserID;

        IF (@UserName IS NOT NULL AND @UserAvailable = 1)
        BEGIN
            -- Compare the provided password with the stored password
            IF (@StoredPassword = @UserPass)
            BEGIN
                -- Authentication successful
                SET @AuthenticationStatus = 1; -- Set to true (1) for success
            END
            ELSE
            BEGIN
                -- Incorrect password
                SET @AuthenticationStatus = 0; -- Set to false (0) for failure
            END
        END
        ELSE IF (@UserAvailable = 0)
        BEGIN
            -- User is not available
            SET @AuthenticationStatus = 0; -- Set to false (0) for failure
        END
        ELSE
        BEGIN
            -- User not found
            SET @AuthenticationStatus = 0; -- Set to false (0) for failure
        END

        -- Commit the transaction if successful
        COMMIT TRAN;
    END TRY
    BEGIN CATCH
        -- Rollback the transaction in case of an error
        ROLLBACK TRAN;
        -- Handle the error or log it as needed
        SET @AuthenticationStatus = 0; -- Set to false (0) for failure
    END CATCH

    -- Return the result as a row
    SELECT @AuthenticationStatus AS AuthenticationStatus, @UserNum AS UserNumber;
END;

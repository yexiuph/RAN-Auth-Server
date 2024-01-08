USE [RanUser]
GO

-- Drop the stored procedure if it already exists
IF OBJECT_ID('[dbo].[ClassicAuth]', 'P') IS NOT NULL
    DROP PROCEDURE [dbo].[ClassicAuth]
GO

-- Create the stored procedure
CREATE PROCEDURE [dbo].[ClassicAuth] (
    @UserID VARCHAR(20),
    @UserPass VARCHAR(20)
)
AS
BEGIN
    SET NOCOUNT ON;

    DECLARE @UserName VARCHAR(20);
    DECLARE @StoredPassword VARCHAR(20);
    DECLARE @UserAvailable INT;
    DECLARE @AuthenticationStatus BIT;
    DECLARE @UserNum INT;

    -- Check if the user exists and is available
    SELECT @UserNum = UserNum, @UserName = UserID, @StoredPassword = UserPass, @UserAvailable = UserAvailable
    FROM UserInfo
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

    -- Return the result as a row
    SELECT @AuthenticationStatus AS AuthenticationStatus, @UserNum AS UserNumber;
END;
GO

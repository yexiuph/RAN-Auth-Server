USE [RanUser]
GO
/****** Object:  StoredProcedure [dbo].[ClassicRegister]    Script Date: 1/8/2024 4:58:38 PM ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

CREATE PROCEDURE [dbo].[ClassicRegister]
    @userId     varchar(25),
    @userMail   varchar(25),
    @userPass1  varchar(50),
    @userPass2  varchar(25)
AS        
BEGIN
    SET NOCOUNT ON

    DECLARE @nReturn INT;

    -- Check if the user already exists
    IF EXISTS (SELECT 1 FROM UserInfo WHERE UserID = @userId)
    BEGIN
        -- User already exists
        SET @nReturn = 0;
    END
    ELSE
    BEGIN
        -- User does not exist, proceed with insertion
        BEGIN TRAN;

        INSERT INTO UserInfo (UserID, UserEmail, UserPass, UserPIN)
        VALUES (@userId, @userMail, @userPass1, @userPass2);

        IF @@ERROR <> 0 OR @@ROWCOUNT = 0
        BEGIN
            ROLLBACK TRAN;
            SET @nReturn = -1;
        END
        ELSE
        BEGIN
            COMMIT TRAN;
            SET @nReturn = 1;
        END
    END

    SET NOCOUNT OFF;

    -- Return the result as a row
    SELECT @nReturn AS RegisterStatus;
END;

USE [RanUser]
GO
/****** Object:  StoredProcedure [dbo].[GSRegister]    Script Date: 1/8/2024 6:06:33 PM ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

ALTER PROCEDURE [dbo].[GSRegister]
    @userId     VARCHAR(25),
    @userMail   VARCHAR(25),
    @userPass1  VARCHAR(50),
    @userPass2  VARCHAR(25)
AS        
BEGIN
    SET NOCOUNT ON

    DECLARE @nReturn INT;

    -- Check if the user already exists
    IF EXISTS (SELECT 1 FROM GSUserInfo WHERE UserID = @userId)
    BEGIN
        -- User already exists
        SET @nReturn = 0;
    END
    ELSE
    BEGIN
        -- User does not exist, proceed with insertion
        BEGIN TRY
            BEGIN TRAN;

            INSERT INTO GSUserInfo (UserID, UserEmail, UserPass, UserPIN)
            VALUES (@userId, @userMail, @userPass1, @userPass2);

            IF @@ERROR <> 0 OR @@ROWCOUNT = 0
            BEGIN
                -- Rollback the transaction if an error occurs during insertion
                ROLLBACK TRAN;
                SET @nReturn = -1;
            END
            ELSE
            BEGIN
                -- Commit the transaction if successful
                COMMIT TRAN;
                SET @nReturn = 1;
            END
        END TRY
        BEGIN CATCH
            -- Handle the error or log it as needed
            SET @nReturn = -1;
            -- Rollback the transaction in case of an error
            ROLLBACK TRAN;
        END CATCH
    END

    SET NOCOUNT OFF;

    -- Return the result as a row
    SELECT @nReturn AS RegisterStatus;
END;

#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADD` reader - Slave address bit 0 (master mode) In 7-bit addressing mode (ADD10 = 0): This bit is dont care In 10-bit addressing mode (ADD10 = 1): This bit should be written with bit 0 of the slave address to be sent Note: Changing these bits when the START bit is set is not allowed."]
pub type SADD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SADD` writer - Slave address bit 0 (master mode) In 7-bit addressing mode (ADD10 = 0): This bit is dont care In 10-bit addressing mode (ADD10 = 1): This bit should be written with bit 0 of the slave address to be sent Note: Changing these bits when the START bit is set is not allowed."]
pub type SADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u16, u16, 10, O>;
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type RD_WRN_R = crate::BitReader<RD_WRN_A>;
#[doc = "Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_WRN_A {
    #[doc = "0: Master requests a write transfer"]
    Write = 0,
    #[doc = "1: Master requests a read transfer"]
    Read = 1,
}
impl From<RD_WRN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_WRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_WRN_A {
        match self.bits {
            false => RD_WRN_A::Write,
            true => RD_WRN_A::Read,
        }
    }
    #[doc = "Checks if the value of the field is `Write`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == RD_WRN_A::Write
    }
    #[doc = "Checks if the value of the field is `Read`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == RD_WRN_A::Read
    }
}
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type RD_WRN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RD_WRN_A, O>;
impl<'a, const O: u8> RD_WRN_W<'a, O> {
    #[doc = "Master requests a write transfer"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(RD_WRN_A::Write)
    }
    #[doc = "Master requests a read transfer"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RD_WRN_A::Read)
    }
}
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type ADD10_R = crate::BitReader<ADD10_A>;
#[doc = "10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD10_A {
    #[doc = "0: The master operates in 7-bit addressing mode"]
    Bit7 = 0,
    #[doc = "1: The master operates in 10-bit addressing mode"]
    Bit10 = 1,
}
impl From<ADD10_A> for bool {
    #[inline(always)]
    fn from(variant: ADD10_A) -> Self {
        variant as u8 != 0
    }
}
impl ADD10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADD10_A {
        match self.bits {
            false => ADD10_A::Bit7,
            true => ADD10_A::Bit10,
        }
    }
    #[doc = "Checks if the value of the field is `Bit7`"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADD10_A::Bit7
    }
    #[doc = "Checks if the value of the field is `Bit10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == ADD10_A::Bit10
    }
}
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type ADD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ADD10_A, O>;
impl<'a, const O: u8> ADD10_W<'a, O> {
    #[doc = "The master operates in 7-bit addressing mode"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADD10_A::Bit7)
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(ADD10_A::Bit10)
    }
}
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type HEAD10R_R = crate::BitReader<HEAD10R_A>;
#[doc = "10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEAD10R_A {
    #[doc = "0: The master sends the complete 10 bit slave address read sequence"]
    Complete = 0,
    #[doc = "1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    Partial = 1,
}
impl From<HEAD10R_A> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R_A) -> Self {
        variant as u8 != 0
    }
}
impl HEAD10R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEAD10R_A {
        match self.bits {
            false => HEAD10R_A::Complete,
            true => HEAD10R_A::Partial,
        }
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == HEAD10R_A::Complete
    }
    #[doc = "Checks if the value of the field is `Partial`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == HEAD10R_A::Partial
    }
}
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type HEAD10R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, HEAD10R_A, O>;
impl<'a, const O: u8> HEAD10R_W<'a, O> {
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(HEAD10R_A::Complete)
    }
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(HEAD10R_A::Partial)
    }
}
#[doc = "Field `START` reader - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free. Note: Writing 0 to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
pub type START_R = crate::BitReader<STARTR_A>;
#[doc = "Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free. Note: Writing 0 to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTR_A {
    #[doc = "0: No Start generation"]
    NoStart = 0,
    #[doc = "1: Restart/Start generation"]
    Start = 1,
}
impl From<STARTR_A> for bool {
    #[inline(always)]
    fn from(variant: STARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTR_A {
        match self.bits {
            false => STARTR_A::NoStart,
            true => STARTR_A::Start,
        }
    }
    #[doc = "Checks if the value of the field is `NoStart`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == STARTR_A::NoStart
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTR_A::Start
    }
}
#[doc = "Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free. Note: Writing 0 to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW_AW {
    #[doc = "1: Restart/Start generation"]
    Start = 1,
}
impl From<STARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: STARTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free. Note: Writing 0 to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
pub type START_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CR2_SPEC, STARTW_AW, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Restart/Start generation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW_AW::Start)
    }
}
#[doc = "Field `STOP` reader - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: Note: Writing 0 to this bit has no effect."]
pub type STOP_R = crate::BitReader<STOPR_A>;
#[doc = "Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: Note: Writing 0 to this bit has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPR_A {
    #[doc = "0: No Stop generation"]
    NoStop = 0,
    #[doc = "1: Stop generation after current byte transfer"]
    Stop = 1,
}
impl From<STOPR_A> for bool {
    #[inline(always)]
    fn from(variant: STOPR_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPR_A {
        match self.bits {
            false => STOPR_A::NoStop,
            true => STOPR_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `NoStop`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPR_A::NoStop
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPR_A::Stop
    }
}
#[doc = "Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: Note: Writing 0 to this bit has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPW_AW {
    #[doc = "1: Stop generation after current byte transfer"]
    Stop = 1,
}
impl From<STOPW_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` writer - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: Note: Writing 0 to this bit has no effect."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CR2_SPEC, STOPW_AW, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    #[doc = "Stop generation after current byte transfer"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPW_AW::Stop)
    }
}
#[doc = "Field `NACK` reader - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing 0 to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
pub type NACK_R = crate::BitReader<NACKR_A>;
#[doc = "NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing 0 to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKR_A {
    #[doc = "0: an ACK is sent after current received byte"]
    Ack = 0,
    #[doc = "1: a NACK is sent after current received byte"]
    Nack = 1,
}
impl From<NACKR_A> for bool {
    #[inline(always)]
    fn from(variant: NACKR_A) -> Self {
        variant as u8 != 0
    }
}
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKR_A {
        match self.bits {
            false => NACKR_A::Ack,
            true => NACKR_A::Nack,
        }
    }
    #[doc = "Checks if the value of the field is `Ack`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == NACKR_A::Ack
    }
    #[doc = "Checks if the value of the field is `Nack`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKR_A::Nack
    }
}
#[doc = "NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing 0 to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKW_AW {
    #[doc = "1: a NACK is sent after current received byte"]
    Nack = 1,
}
impl From<NACKW_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` writer - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing 0 to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
pub type NACK_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CR2_SPEC, NACKW_AW, O>;
impl<'a, const O: u8> NACK_W<'a, O> {
    #[doc = "a NACK is sent after current received byte"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(NACKW_AW::Nack)
    }
}
#[doc = "Field `NBYTES` reader - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
pub type NBYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBYTES` writer - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
pub type NBYTES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `RELOAD` reader - NBYTES reload mode This bit is set and cleared by software."]
pub type RELOAD_R = crate::BitReader<RELOAD_A>;
#[doc = "NBYTES reload mode This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD_A {
    #[doc = "0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    Completed = 0,
    #[doc = "1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    NotCompleted = 1,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl RELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            false => RELOAD_A::Completed,
            true => RELOAD_A::NotCompleted,
        }
    }
    #[doc = "Checks if the value of the field is `Completed`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == RELOAD_A::Completed
    }
    #[doc = "Checks if the value of the field is `NotCompleted`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == RELOAD_A::NotCompleted
    }
}
#[doc = "Field `RELOAD` writer - NBYTES reload mode This bit is set and cleared by software."]
pub type RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RELOAD_A, O>;
impl<'a, const O: u8> RELOAD_W<'a, O> {
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(RELOAD_A::Completed)
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut W {
        self.variant(RELOAD_A::NotCompleted)
    }
}
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
pub type AUTOEND_R = crate::BitReader<AUTOEND_A>;
#[doc = "Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOEND_A {
    #[doc = "0: Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    Software = 0,
    #[doc = "1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    Automatic = 1,
}
impl From<AUTOEND_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOEND_A {
        match self.bits {
            false => AUTOEND_A::Software,
            true => AUTOEND_A::Automatic,
        }
    }
    #[doc = "Checks if the value of the field is `Software`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == AUTOEND_A::Software
    }
    #[doc = "Checks if the value of the field is `Automatic`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AUTOEND_A::Automatic
    }
}
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
pub type AUTOEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, AUTOEND_A, O>;
impl<'a, const O: u8> AUTOEND_W<'a, O> {
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(AUTOEND_A::Software)
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTOEND_A::Automatic)
    }
}
#[doc = "Field `PECBYTE` reader - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing 0 to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
pub type PECBYTE_R = crate::BitReader<PECBYTER_A>;
#[doc = "Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing 0 to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTER_A {
    #[doc = "0: No PEC transfer"]
    NoPec = 0,
    #[doc = "1: PEC transmission/reception is requested"]
    Pec = 1,
}
impl From<PECBYTER_A> for bool {
    #[inline(always)]
    fn from(variant: PECBYTER_A) -> Self {
        variant as u8 != 0
    }
}
impl PECBYTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECBYTER_A {
        match self.bits {
            false => PECBYTER_A::NoPec,
            true => PECBYTER_A::Pec,
        }
    }
    #[doc = "Checks if the value of the field is `NoPec`"]
    #[inline(always)]
    pub fn is_no_pec(&self) -> bool {
        *self == PECBYTER_A::NoPec
    }
    #[doc = "Checks if the value of the field is `Pec`"]
    #[inline(always)]
    pub fn is_pec(&self) -> bool {
        *self == PECBYTER_A::Pec
    }
}
#[doc = "Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing 0 to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTEW_AW {
    #[doc = "1: PEC transmission/reception is requested"]
    Pec = 1,
}
impl From<PECBYTEW_AW> for bool {
    #[inline(always)]
    fn from(variant: PECBYTEW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECBYTE` writer - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing 0 to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
pub type PECBYTE_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CR2_SPEC, PECBYTEW_AW, O>;
impl<'a, const O: u8> PECBYTE_W<'a, O> {
    #[doc = "PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn pec(self) -> &'a mut W {
        self.variant(PECBYTEW_AW::Pec)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave address bit 0 (master mode) In 7-bit addressing mode (ADD10 = 0): This bit is dont care In 10-bit addressing mode (ADD10 = 1): This bit should be written with bit 0 of the slave address to be sent Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free. Note: Writing 0 to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: Note: Writing 0 to this bit has no effect."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing 0 to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing 0 to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address bit 0 (master mode) In 7-bit addressing mode (ADD10 = 0): This bit is dont care In 10-bit addressing mode (ADD10 = 1): This bit should be written with bit 0 of the slave address to be sent Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn sadd(&mut self) -> SADD_W<0> {
        SADD_W::new(self)
    }
    #[doc = "Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<10> {
        RD_WRN_W::new(self)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<11> {
        ADD10_W::new(self)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<12> {
        HEAD10R_W::new(self)
    }
    #[doc = "Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0. It can also be cleared by software by writing 1 to the ADDRCF bit in the I2C_ICR register. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit will generate a START condition once the bus is free. Note: Writing 0 to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<13> {
        START_W::new(self)
    }
    #[doc = "Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a Stop condition is detected, or when PE = 0. In Master Mode: Note: Writing 0 to this bit has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<14> {
        STOP_W::new(self)
    }
    #[doc = "Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing 0 to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<15> {
        NACK_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is dont care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<16> {
        NBYTES_W::new(self)
    }
    #[doc = "Bit 24 - NBYTES reload mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<24> {
        RELOAD_W::new(self)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<25> {
        AUTOEND_W::new(self)
    }
    #[doc = "Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing 0 to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
    #[inline(always)]
    #[must_use]
    pub fn pecbyte(&mut self) -> PECBYTE_W<26> {
        PECBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0400_e000;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

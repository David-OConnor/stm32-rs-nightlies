#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRCF_AW {
    #[doc = "1: Clears the ADDR flag in ISR register"]
    Clear = 1,
}
impl From<ADDRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRCF` writer - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
pub type ADDRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ADDRCF_AW, O>;
impl<'a, const O: u8> ADDRCF_W<'a, O> {
    #[doc = "Clears the ADDR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRCF_AW::Clear)
    }
}
#[doc = "Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKCF_AW {
    #[doc = "1: Clears the NACK flag in ISR register"]
    Clear = 1,
}
impl From<NACKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKCF` writer - Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register."]
pub type NACKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, NACKCF_AW, O>;
impl<'a, const O: u8> NACKCF_W<'a, O> {
    #[doc = "Clears the NACK flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NACKCF_AW::Clear)
    }
}
#[doc = "Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPCF_AW {
    #[doc = "1: Clears the STOP flag in ISR register"]
    Clear = 1,
}
impl From<STOPCF_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPCF` writer - Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
pub type STOPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, STOPCF_AW, O>;
impl<'a, const O: u8> STOPCF_W<'a, O> {
    #[doc = "Clears the STOP flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPCF_AW::Clear)
    }
}
#[doc = "Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRCF_AW {
    #[doc = "1: Clears the BERR flag in ISR register"]
    Clear = 1,
}
impl From<BERRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: BERRCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERRCF` writer - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
pub type BERRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, BERRCF_AW, O>;
impl<'a, const O: u8> BERRCF_W<'a, O> {
    #[doc = "Clears the BERR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BERRCF_AW::Clear)
    }
}
#[doc = "Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOCF_AW {
    #[doc = "1: Clears the ARLO flag in ISR register"]
    Clear = 1,
}
impl From<ARLOCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARLOCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLOCF` writer - Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
pub type ARLOCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ARLOCF_AW, O>;
impl<'a, const O: u8> ARLOCF_W<'a, O> {
    #[doc = "Clears the ARLO flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARLOCF_AW::Clear)
    }
}
#[doc = "Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCF_AW {
    #[doc = "1: Clears the OVR flag in ISR register"]
    Clear = 1,
}
impl From<OVRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRCF` writer - Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
pub type OVRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, OVRCF_AW, O>;
impl<'a, const O: u8> OVRCF_W<'a, O> {
    #[doc = "Clears the OVR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRCF_AW::Clear)
    }
}
#[doc = "PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECCF_AW {
    #[doc = "1: Clears the PEC flag in ISR register"]
    Clear = 1,
}
impl From<PECCF_AW> for bool {
    #[inline(always)]
    fn from(variant: PECCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECCF` writer - PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
pub type PECCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, PECCF_AW, O>;
impl<'a, const O: u8> PECCF_W<'a, O> {
    #[doc = "Clears the PEC flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECCF_AW::Clear)
    }
}
#[doc = "Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTCF_AW {
    #[doc = "1: Clears the TIMOUT flag in ISR register"]
    Clear = 1,
}
impl From<TIMOUTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUTCF` writer - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
pub type TIMOUTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, TIMOUTCF_AW, O>;
impl<'a, const O: u8> TIMOUTCF_W<'a, O> {
    #[doc = "Clears the TIMOUT flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIMOUTCF_AW::Clear)
    }
}
#[doc = "Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTCF_AW {
    #[doc = "1: Clears the ALERT flag in ISR register"]
    Clear = 1,
}
impl From<ALERTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALERTCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTCF` writer - Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
pub type ALERTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ALERTCF_AW, O>;
impl<'a, const O: u8> ALERTCF_W<'a, O> {
    #[doc = "Clears the ALERT flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALERTCF_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 3 - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
    #[inline(always)]
    #[must_use]
    pub fn addrcf(&mut self) -> ADDRCF_W<3> {
        ADDRCF_W::new(self)
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn nackcf(&mut self) -> NACKCF_W<4> {
        NACKCF_W::new(self)
    }
    #[doc = "Bit 5 - Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn stopcf(&mut self) -> STOPCF_W<5> {
        STOPCF_W::new(self)
    }
    #[doc = "Bit 8 - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn berrcf(&mut self) -> BERRCF_W<8> {
        BERRCF_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn arlocf(&mut self) -> ARLOCF_W<9> {
        ARLOCF_W::new(self)
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<10> {
        OVRCF_W::new(self)
    }
    #[doc = "Bit 11 - PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
    #[inline(always)]
    #[must_use]
    pub fn peccf(&mut self) -> PECCF_W<11> {
        PECCF_W::new(self)
    }
    #[doc = "Bit 12 - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
    #[inline(always)]
    #[must_use]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<12> {
        TIMOUTCF_W::new(self)
    }
    #[doc = "Bit 13 - Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
    #[inline(always)]
    #[must_use]
    pub fn alertcf(&mut self) -> ALERTCF_W<13> {
        ALERTCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

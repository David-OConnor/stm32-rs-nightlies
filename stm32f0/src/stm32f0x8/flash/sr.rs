#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BSY` reader - Busy"]
pub type BSY_R = crate::BitReader<BSYR_A>;
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR_A {
    #[doc = "0: No write/erase operation is in progress"]
    Inactive = 0,
    #[doc = "1: A write/erase operation is in progress"]
    Active = 1,
}
impl From<BSYR_A> for bool {
    #[inline(always)]
    fn from(variant: BSYR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSYR_A {
        match self.bits {
            false => BSYR_A::Inactive,
            true => BSYR_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `Inactive`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSYR_A::Inactive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSYR_A::Active
    }
}
#[doc = "Field `PGERR` reader - Programming error"]
pub type PGERR_R = crate::BitReader<PGERR_A>;
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERR_A {
    #[doc = "0: No programming error occurred"]
    NoError = 0,
    #[doc = "1: A programming error occurred"]
    Error = 1,
}
impl From<PGERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGERR_A {
        match self.bits {
            false => PGERR_A::NoError,
            true => PGERR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERR_A::Error
    }
}
#[doc = "Field `PGERR` writer - Programming error"]
pub type PGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, PGERR_A, O>;
impl<'a, const O: u8> PGERR_W<'a, O> {
    #[doc = "No programming error occurred"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(PGERR_A::NoError)
    }
    #[doc = "A programming error occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(PGERR_A::Error)
    }
}
#[doc = "Field `WRPRT` reader - Write protection error"]
pub type WRPRT_R = crate::BitReader<WRPRT_A>;
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPRT_A {
    #[doc = "0: No write protection error occurred"]
    NoError = 0,
    #[doc = "1: A write protection error occurred"]
    Error = 1,
}
impl From<WRPRT_A> for bool {
    #[inline(always)]
    fn from(variant: WRPRT_A) -> Self {
        variant as u8 != 0
    }
}
impl WRPRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPRT_A {
        match self.bits {
            false => WRPRT_A::NoError,
            true => WRPRT_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPRT_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPRT_A::Error
    }
}
#[doc = "Field `WRPRT` writer - Write protection error"]
pub type WRPRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, WRPRT_A, O>;
impl<'a, const O: u8> WRPRT_W<'a, O> {
    #[doc = "No write protection error occurred"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(WRPRT_A::NoError)
    }
    #[doc = "A write protection error occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(WRPRT_A::Error)
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader<EOP_A>;
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOP_A {
    #[doc = "0: No EOP operation occurred"]
    NoEvent = 0,
    #[doc = "1: An EOP event occurred"]
    Event = 1,
}
impl From<EOP_A> for bool {
    #[inline(always)]
    fn from(variant: EOP_A) -> Self {
        variant as u8 != 0
    }
}
impl EOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOP_A {
        match self.bits {
            false => EOP_A::NoEvent,
            true => EOP_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOP_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOP_A::Event
    }
}
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, EOP_A, O>;
impl<'a, const O: u8> EOP_W<'a, O> {
    #[doc = "No EOP operation occurred"]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EOP_A::NoEvent)
    }
    #[doc = "An EOP event occurred"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(EOP_A::Event)
    }
}
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprt(&self) -> WRPRT_R {
        WRPRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    #[must_use]
    pub fn pgerr(&mut self) -> PGERR_W<2> {
        PGERR_W::new(self)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    #[must_use]
    pub fn wrprt(&mut self) -> WRPRT_W<4> {
        WRPRT_W::new(self)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<5> {
        EOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

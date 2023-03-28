#[doc = "Register `RFDCR` reader"]
pub struct R(crate::R<RFDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFDCR` writer"]
pub struct W(crate::W<RFDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFDCR_SPEC>;
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
impl From<crate::W<RFDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFTBSEL` reader - radio debug test bus selection"]
pub type RFTBSEL_R = crate::BitReader<RFTBSEL_A>;
#[doc = "radio debug test bus selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFTBSEL_A {
    #[doc = "0: Digital test bus selected on RF_ADTB\\[3:0\\]"]
    Digital = 0,
    #[doc = "1: Analog test bus selected on RF_ADTB\\[3:0\\]"]
    Analog = 1,
}
impl From<RFTBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RFTBSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RFTBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFTBSEL_A {
        match self.bits {
            false => RFTBSEL_A::Digital,
            true => RFTBSEL_A::Analog,
        }
    }
    #[doc = "Checks if the value of the field is `Digital`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == RFTBSEL_A::Digital
    }
    #[doc = "Checks if the value of the field is `Analog`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == RFTBSEL_A::Analog
    }
}
#[doc = "Field `RFTBSEL` writer - radio debug test bus selection"]
pub type RFTBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFDCR_SPEC, RFTBSEL_A, O>;
impl<'a, const O: u8> RFTBSEL_W<'a, O> {
    #[doc = "Digital test bus selected on RF_ADTB\\[3:0\\]"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(RFTBSEL_A::Digital)
    }
    #[doc = "Analog test bus selected on RF_ADTB\\[3:0\\]"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(RFTBSEL_A::Analog)
    }
}
impl R {
    #[doc = "Bit 0 - radio debug test bus selection"]
    #[inline(always)]
    pub fn rftbsel(&self) -> RFTBSEL_R {
        RFTBSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - radio debug test bus selection"]
    #[inline(always)]
    #[must_use]
    pub fn rftbsel(&mut self) -> RFTBSEL_W<0> {
        RFTBSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "radio debug control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfdcr](index.html) module"]
pub struct RFDCR_SPEC;
impl crate::RegisterSpec for RFDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfdcr::R](R) reader structure"]
impl crate::Readable for RFDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfdcr::W](W) writer structure"]
impl crate::Writable for RFDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFDCR to value 0"]
impl crate::Resettable for RFDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

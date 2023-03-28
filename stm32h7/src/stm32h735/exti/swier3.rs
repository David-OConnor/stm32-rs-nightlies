#[doc = "Register `SWIER3` reader"]
pub struct R(crate::R<SWIER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER3` writer"]
pub struct W(crate::W<SWIER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER3_SPEC>;
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
impl From<crate::W<SWIER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWIER82` reader - Software interrupt on line x+64"]
pub type SWIER82_R = crate::BitReader<SWIER82W_A>;
#[doc = "Software interrupt on line x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIER82W_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER82W_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER82W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWIER82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIER82W_A> {
        match self.bits {
            true => Some(SWIER82W_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER82W_A::Pend
    }
}
#[doc = "Field `SWIER82` writer - Software interrupt on line x+64"]
pub type SWIER82_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER3_SPEC, SWIER82W_A, O>;
impl<'a, const O: u8> SWIER82_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER82W_A::Pend)
    }
}
#[doc = "Field `SWIER84` reader - Software interrupt on line x+64"]
pub use SWIER82_R as SWIER84_R;
#[doc = "Field `SWIER85` reader - Software interrupt on line x+64"]
pub use SWIER82_R as SWIER85_R;
#[doc = "Field `SWIER86` reader - Software interrupt on line x+64"]
pub use SWIER82_R as SWIER86_R;
#[doc = "Field `SWIER84` writer - Software interrupt on line x+64"]
pub use SWIER82_W as SWIER84_W;
#[doc = "Field `SWIER85` writer - Software interrupt on line x+64"]
pub use SWIER82_W as SWIER85_W;
#[doc = "Field `SWIER86` writer - Software interrupt on line x+64"]
pub use SWIER82_W as SWIER86_W;
impl R {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&self) -> SWIER82_R {
        SWIER82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&self) -> SWIER84_R {
        SWIER84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&self) -> SWIER85_R {
        SWIER85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&self) -> SWIER86_R {
        SWIER86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier82(&mut self) -> SWIER82_W<18> {
        SWIER82_W::new(self)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier84(&mut self) -> SWIER84_W<20> {
        SWIER84_W::new(self)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier85(&mut self) -> SWIER85_W<21> {
        SWIER85_W::new(self)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier86(&mut self) -> SWIER86_W<22> {
        SWIER86_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier3](index.html) module"]
pub struct SWIER3_SPEC;
impl crate::RegisterSpec for SWIER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier3::R](R) reader structure"]
impl crate::Readable for SWIER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier3::W](W) writer structure"]
impl crate::Writable for SWIER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIER3 to value 0"]
impl crate::Resettable for SWIER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

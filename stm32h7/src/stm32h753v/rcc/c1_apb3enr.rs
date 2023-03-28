#[doc = "Register `C1_APB3ENR` reader"]
pub struct R(crate::R<C1_APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB3ENR` writer"]
pub struct W(crate::W<C1_APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB3ENR_SPEC>;
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
impl From<crate::W<C1_APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTDCEN` reader - LTDC peripheral clock enable"]
pub type LTDCEN_R = crate::BitReader<LTDCEN_A>;
#[doc = "LTDC peripheral clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<LTDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LTDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCEN_A {
        match self.bits {
            false => LTDCEN_A::Disabled,
            true => LTDCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN_A::Enabled
    }
}
#[doc = "Field `LTDCEN` writer - LTDC peripheral clock enable"]
pub type LTDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_APB3ENR_SPEC, LTDCEN_A, O>;
impl<'a, const O: u8> LTDCEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::Enabled)
    }
}
#[doc = "Field `WWDG1EN` reader - WWDG1 Clock Enable"]
pub use LTDCEN_R as WWDG1EN_R;
#[doc = "Field `WWDG1EN` writer - WWDG1 Clock Enable"]
pub use LTDCEN_W as WWDG1EN_W;
impl R {
    #[doc = "Bit 3 - LTDC peripheral clock enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable"]
    #[inline(always)]
    pub fn wwdg1en(&self) -> WWDG1EN_R {
        WWDG1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<3> {
        LTDCEN_W::new(self)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg1en(&mut self) -> WWDG1EN_W<6> {
        WWDG1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB3 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb3enr](index.html) module"]
pub struct C1_APB3ENR_SPEC;
impl crate::RegisterSpec for C1_APB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb3enr::R](R) reader structure"]
impl crate::Readable for C1_APB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb3enr::W](W) writer structure"]
impl crate::Writable for C1_APB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_APB3ENR to value 0"]
impl crate::Resettable for C1_APB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

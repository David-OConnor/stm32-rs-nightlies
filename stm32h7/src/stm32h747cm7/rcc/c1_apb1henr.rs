#[doc = "Register `C1_APB1HENR` reader"]
pub struct R(crate::R<C1_APB1HENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB1HENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB1HENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB1HENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB1HENR` writer"]
pub struct W(crate::W<C1_APB1HENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB1HENR_SPEC>;
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
impl From<crate::W<C1_APB1HENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB1HENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSEN` reader - Clock Recovery System peripheral clock enable"]
pub type CRSEN_R = crate::BitReader<CRSEN_A>;
#[doc = "Clock Recovery System peripheral clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<CRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSEN_A {
        match self.bits {
            false => CRSEN_A::Disabled,
            true => CRSEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSEN_A::Enabled
    }
}
#[doc = "Field `CRSEN` writer - Clock Recovery System peripheral clock enable"]
pub type CRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_APB1HENR_SPEC, CRSEN_A, O>;
impl<'a, const O: u8> CRSEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSEN_A::Enabled)
    }
}
#[doc = "Field `SWPEN` reader - SWPMI Peripheral Clocks Enable"]
pub use CRSEN_R as SWPEN_R;
#[doc = "Field `OPAMPEN` reader - OPAMP peripheral clock enable"]
pub use CRSEN_R as OPAMPEN_R;
#[doc = "Field `MDIOSEN` reader - MDIOS peripheral clock enable"]
pub use CRSEN_R as MDIOSEN_R;
#[doc = "Field `FDCANEN` reader - FDCAN Peripheral Clocks Enable"]
pub use CRSEN_R as FDCANEN_R;
#[doc = "Field `SWPEN` writer - SWPMI Peripheral Clocks Enable"]
pub use CRSEN_W as SWPEN_W;
#[doc = "Field `OPAMPEN` writer - OPAMP peripheral clock enable"]
pub use CRSEN_W as OPAMPEN_W;
#[doc = "Field `MDIOSEN` writer - MDIOS peripheral clock enable"]
pub use CRSEN_W as MDIOSEN_W;
#[doc = "Field `FDCANEN` writer - FDCAN Peripheral Clocks Enable"]
pub use CRSEN_W as FDCANEN_W;
impl R {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn swpen(&self) -> SWPEN_R {
        SWPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable"]
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable"]
    #[inline(always)]
    pub fn mdiosen(&self) -> MDIOSEN_R {
        MDIOSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<1> {
        CRSEN_W::new(self)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen(&mut self) -> SWPEN_W<2> {
        SWPEN_W::new(self)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OPAMPEN_W<4> {
        OPAMPEN_W::new(self)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdiosen(&mut self) -> MDIOSEN_W<5> {
        MDIOSEN_W::new(self)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<8> {
        FDCANEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1henr](index.html) module"]
pub struct C1_APB1HENR_SPEC;
impl crate::RegisterSpec for C1_APB1HENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb1henr::R](R) reader structure"]
impl crate::Readable for C1_APB1HENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb1henr::W](W) writer structure"]
impl crate::Writable for C1_APB1HENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_APB1HENR to value 0"]
impl crate::Resettable for C1_APB1HENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

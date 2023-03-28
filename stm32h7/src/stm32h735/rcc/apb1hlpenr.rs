#[doc = "Register `APB1HLPENR` reader"]
pub struct R(crate::R<APB1HLPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1HLPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1HLPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1HLPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1HLPENR` writer"]
pub struct W(crate::W<APB1HLPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1HLPENR_SPEC>;
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
impl From<crate::W<APB1HLPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1HLPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSLPEN` reader - Clock Recovery System peripheral clock enable during CSleep mode"]
pub type CRSLPEN_R = crate::BitReader<CRSLPEN_A>;
#[doc = "Clock Recovery System peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSLPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<CRSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRSLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSLPEN_A {
        match self.bits {
            false => CRSLPEN_A::Disabled,
            true => CRSLPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSLPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSLPEN_A::Enabled
    }
}
#[doc = "Field `CRSLPEN` writer - Clock Recovery System peripheral clock enable during CSleep mode"]
pub type CRSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HLPENR_SPEC, CRSLPEN_A, O>;
impl<'a, const O: u8> CRSLPEN_W<'a, O> {
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::Enabled)
    }
}
#[doc = "Field `SWPMILPEN` reader - SWPMI Peripheral Clocks Enable During CSleep Mode"]
pub use CRSLPEN_R as SWPMILPEN_R;
#[doc = "Field `OPAMPLPEN` reader - OPAMP peripheral clock enable during CSleep mode"]
pub use CRSLPEN_R as OPAMPLPEN_R;
#[doc = "Field `MDIOSLPEN` reader - MDIOS peripheral clock enable during CSleep mode"]
pub use CRSLPEN_R as MDIOSLPEN_R;
#[doc = "Field `FDCANLPEN` reader - FDCAN Peripheral Clocks Enable During CSleep Mode"]
pub use CRSLPEN_R as FDCANLPEN_R;
#[doc = "Field `TIM23LPEN` reader - TIM23 block enable during CSleep Mode"]
pub use CRSLPEN_R as TIM23LPEN_R;
#[doc = "Field `TIM24LPEN` reader - TIM24 block enable during CSleep Mode"]
pub use CRSLPEN_R as TIM24LPEN_R;
#[doc = "Field `SWPMILPEN` writer - SWPMI Peripheral Clocks Enable During CSleep Mode"]
pub use CRSLPEN_W as SWPMILPEN_W;
#[doc = "Field `OPAMPLPEN` writer - OPAMP peripheral clock enable during CSleep mode"]
pub use CRSLPEN_W as OPAMPLPEN_W;
#[doc = "Field `MDIOSLPEN` writer - MDIOS peripheral clock enable during CSleep mode"]
pub use CRSLPEN_W as MDIOSLPEN_W;
#[doc = "Field `FDCANLPEN` writer - FDCAN Peripheral Clocks Enable During CSleep Mode"]
pub use CRSLPEN_W as FDCANLPEN_W;
#[doc = "Field `TIM23LPEN` writer - TIM23 block enable during CSleep Mode"]
pub use CRSLPEN_W as TIM23LPEN_W;
#[doc = "Field `TIM24LPEN` writer - TIM24 block enable during CSleep Mode"]
pub use CRSLPEN_W as TIM24LPEN_W;
impl R {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn swpmilpen(&self) -> SWPMILPEN_R {
        SWPMILPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn opamplpen(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 24 - TIM23 block enable during CSleep Mode"]
    #[inline(always)]
    pub fn tim23lpen(&self) -> TIM23LPEN_R {
        TIM23LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TIM24 block enable during CSleep Mode"]
    #[inline(always)]
    pub fn tim24lpen(&self) -> TIM24LPEN_R {
        TIM24LPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crslpen(&mut self) -> CRSLPEN_W<1> {
        CRSLPEN_W::new(self)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn swpmilpen(&mut self) -> SWPMILPEN_W<2> {
        SWPMILPEN_W::new(self)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn opamplpen(&mut self) -> OPAMPLPEN_W<4> {
        OPAMPLPEN_W::new(self)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W<5> {
        MDIOSLPEN_W::new(self)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<8> {
        FDCANLPEN_W::new(self)
    }
    #[doc = "Bit 24 - TIM23 block enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim23lpen(&mut self) -> TIM23LPEN_W<24> {
        TIM23LPEN_W::new(self)
    }
    #[doc = "Bit 25 - TIM24 block enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim24lpen(&mut self) -> TIM24LPEN_W<25> {
        TIM24LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 High Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hlpenr](index.html) module"]
pub struct APB1HLPENR_SPEC;
impl crate::RegisterSpec for APB1HLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1hlpenr::R](R) reader structure"]
impl crate::Readable for APB1HLPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1hlpenr::W](W) writer structure"]
impl crate::Writable for APB1HLPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1HLPENR to value 0"]
impl crate::Resettable for APB1HLPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

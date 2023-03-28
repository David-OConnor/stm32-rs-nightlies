#[doc = "Register `RCC_APB4RSTSETR` reader"]
pub struct R(crate::R<RCC_APB4RSTSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB4RSTSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB4RSTSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB4RSTSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB4RSTSETR` writer"]
pub struct W(crate::W<RCC_APB4RSTSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB4RSTSETR_SPEC>;
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
impl From<crate::W<RCC_APB4RSTSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB4RSTSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTDCRST` reader - LTDCRST"]
pub type LTDCRST_R = crate::BitReader<bool>;
#[doc = "Field `LTDCRST` writer - LTDCRST"]
pub type LTDCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB4RSTSETR_SPEC, bool, O>;
#[doc = "Field `DSIRST` reader - DSIRST"]
pub type DSIRST_R = crate::BitReader<bool>;
#[doc = "Field `DSIRST` writer - DSIRST"]
pub type DSIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB4RSTSETR_SPEC, bool, O>;
#[doc = "Field `DDRPERFMRST` reader - DDRPERFMRST"]
pub type DDRPERFMRST_R = crate::BitReader<bool>;
#[doc = "Field `DDRPERFMRST` writer - DDRPERFMRST"]
pub type DDRPERFMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB4RSTSETR_SPEC, bool, O>;
#[doc = "Field `USBPHYRST` reader - USBPHYRST"]
pub type USBPHYRST_R = crate::BitReader<bool>;
#[doc = "Field `USBPHYRST` writer - USBPHYRST"]
pub type USBPHYRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB4RSTSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    pub fn ddrperfmrst(&self) -> DDRPERFMRST_R {
        DDRPERFMRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    pub fn usbphyrst(&self) -> USBPHYRST_R {
        USBPHYRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<0> {
        LTDCRST_W::new(self)
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    #[must_use]
    pub fn dsirst(&mut self) -> DSIRST_W<4> {
        DSIRST_W::new(self)
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    #[must_use]
    pub fn ddrperfmrst(&mut self) -> DDRPERFMRST_W<8> {
        DDRPERFMRST_W::new(self)
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbphyrst(&mut self) -> USBPHYRST_W<16> {
        USBPHYRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb4rstsetr](index.html) module"]
pub struct RCC_APB4RSTSETR_SPEC;
impl crate::RegisterSpec for RCC_APB4RSTSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb4rstsetr::R](R) reader structure"]
impl crate::Readable for RCC_APB4RSTSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb4rstsetr::W](W) writer structure"]
impl crate::Writable for RCC_APB4RSTSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_APB4RSTSETR to value 0"]
impl crate::Resettable for RCC_APB4RSTSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

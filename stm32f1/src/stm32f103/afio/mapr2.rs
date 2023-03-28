#[doc = "Register `MAPR2` reader"]
pub struct R(crate::R<MAPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPR2` writer"]
pub struct W(crate::W<MAPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPR2_SPEC>;
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
impl From<crate::W<MAPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM9_REMAP` reader - TIM9 remapping"]
pub type TIM9_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM9_REMAP` writer - TIM9 remapping"]
pub type TIM9_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
#[doc = "Field `TIM10_REMAP` reader - TIM10 remapping"]
pub type TIM10_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM10_REMAP` writer - TIM10 remapping"]
pub type TIM10_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
#[doc = "Field `TIM11_REMAP` reader - TIM11 remapping"]
pub type TIM11_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM11_REMAP` writer - TIM11 remapping"]
pub type TIM11_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
#[doc = "Field `TIM13_REMAP` reader - TIM13 remapping"]
pub type TIM13_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM13_REMAP` writer - TIM13 remapping"]
pub type TIM13_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
#[doc = "Field `TIM14_REMAP` reader - TIM14 remapping"]
pub type TIM14_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM14_REMAP` writer - TIM14 remapping"]
pub type TIM14_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
#[doc = "Field `FSMC_NADV` reader - NADV connect/disconnect"]
pub type FSMC_NADV_R = crate::BitReader<bool>;
#[doc = "Field `FSMC_NADV` writer - NADV connect/disconnect"]
pub type FSMC_NADV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - TIM9 remapping"]
    #[inline(always)]
    pub fn tim9_remap(&self) -> TIM9_REMAP_R {
        TIM9_REMAP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM10 remapping"]
    #[inline(always)]
    pub fn tim10_remap(&self) -> TIM10_REMAP_R {
        TIM10_REMAP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM11 remapping"]
    #[inline(always)]
    pub fn tim11_remap(&self) -> TIM11_REMAP_R {
        TIM11_REMAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - TIM9 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim9_remap(&mut self) -> TIM9_REMAP_W<5> {
        TIM9_REMAP_W::new(self)
    }
    #[doc = "Bit 6 - TIM10 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim10_remap(&mut self) -> TIM10_REMAP_W<6> {
        TIM10_REMAP_W::new(self)
    }
    #[doc = "Bit 7 - TIM11 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim11_remap(&mut self) -> TIM11_REMAP_W<7> {
        TIM11_REMAP_W::new(self)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W<8> {
        TIM13_REMAP_W::new(self)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W<9> {
        TIM14_REMAP_W::new(self)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W<10> {
        FSMC_NADV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AF remap and debug I/O configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapr2](index.html) module"]
pub struct MAPR2_SPEC;
impl crate::RegisterSpec for MAPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapr2::R](R) reader structure"]
impl crate::Readable for MAPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapr2::W](W) writer structure"]
impl crate::Writable for MAPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPR2 to value 0"]
impl crate::Resettable for MAPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "global interrupt flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF1_AW {
    #[doc = "1: Clear the corresponding CGIFx flag"]
    Clear = 1,
}
impl From<GIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: GIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF1` writer - global interrupt flag clear for channel 1"]
pub type GIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, GIF1_AW, O>;
impl<'a, const O: u8> GIF1_W<'a, O> {
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GIF1_AW::Clear)
    }
}
#[doc = "transfer complete flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF1_AW {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    Clear = 1,
}
impl From<TCIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: TCIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF1` writer - transfer complete flag clear for channel 1"]
pub type TCIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, TCIF1_AW, O>;
impl<'a, const O: u8> TCIF1_W<'a, O> {
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCIF1_AW::Clear)
    }
}
#[doc = "half transfer flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF1_AW {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    Clear = 1,
}
impl From<HTIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: HTIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF1` writer - half transfer flag clear for channel 1"]
pub type HTIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, HTIF1_AW, O>;
impl<'a, const O: u8> HTIF1_W<'a, O> {
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HTIF1_AW::Clear)
    }
}
#[doc = "transfer error flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF1_AW {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    Clear = 1,
}
impl From<TEIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: TEIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF1` writer - transfer error flag clear for channel 1"]
pub type TEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, TEIF1_AW, O>;
impl<'a, const O: u8> TEIF1_W<'a, O> {
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TEIF1_AW::Clear)
    }
}
#[doc = "Field `GIF2` writer - global interrupt flag clear for channel 2"]
pub use GIF1_W as GIF2_W;
#[doc = "Field `GIF3` writer - global interrupt flag clear for channel 3"]
pub use GIF1_W as GIF3_W;
#[doc = "Field `GIF4` writer - global interrupt flag clear for channel 4"]
pub use GIF1_W as GIF4_W;
#[doc = "Field `GIF5` writer - global interrupt flag clear for channel 5"]
pub use GIF1_W as GIF5_W;
#[doc = "Field `GIF6` writer - global interrupt flag clear for channel 6"]
pub use GIF1_W as GIF6_W;
#[doc = "Field `GIF7` writer - global interrupt flag clear for channel 7"]
pub use GIF1_W as GIF7_W;
#[doc = "Field `HTIF2` writer - half transfer flag clear for channel 2"]
pub use HTIF1_W as HTIF2_W;
#[doc = "Field `HTIF3` writer - half transfer flag clear for channel 3"]
pub use HTIF1_W as HTIF3_W;
#[doc = "Field `HTIF4` writer - half transfer flag clear for channel 4"]
pub use HTIF1_W as HTIF4_W;
#[doc = "Field `HTIF5` writer - half transfer flag clear for channel 5"]
pub use HTIF1_W as HTIF5_W;
#[doc = "Field `HTIF6` writer - half transfer flag clear for channel 6"]
pub use HTIF1_W as HTIF6_W;
#[doc = "Field `HTIF7` writer - half transfer flag clear for channel 7"]
pub use HTIF1_W as HTIF7_W;
#[doc = "Field `TCIF2` writer - transfer complete flag clear for channel 2"]
pub use TCIF1_W as TCIF2_W;
#[doc = "Field `TCIF3` writer - transfer complete flag clear for channel 3"]
pub use TCIF1_W as TCIF3_W;
#[doc = "Field `TCIF4` writer - transfer complete flag clear for channel 4"]
pub use TCIF1_W as TCIF4_W;
#[doc = "Field `TCIF5` writer - transfer complete flag clear for channel 5"]
pub use TCIF1_W as TCIF5_W;
#[doc = "Field `TCIF6` writer - transfer complete flag clear for channel 6"]
pub use TCIF1_W as TCIF6_W;
#[doc = "Field `TCIF7` writer - transfer complete flag clear for channel 7"]
pub use TCIF1_W as TCIF7_W;
#[doc = "Field `TEIF2` writer - transfer error flag clear for channel 2"]
pub use TEIF1_W as TEIF2_W;
#[doc = "Field `TEIF3` writer - transfer error flag clear for channel 3"]
pub use TEIF1_W as TEIF3_W;
#[doc = "Field `TEIF4` writer - transfer error flag clear for channel 4"]
pub use TEIF1_W as TEIF4_W;
#[doc = "Field `TEIF5` writer - transfer error flag clear for channel 5"]
pub use TEIF1_W as TEIF5_W;
#[doc = "Field `TEIF6` writer - transfer error flag clear for channel 6"]
pub use TEIF1_W as TEIF6_W;
#[doc = "Field `TEIF7` writer - transfer error flag clear for channel 7"]
pub use TEIF1_W as TEIF7_W;
impl W {
    #[doc = "Bit 0 - global interrupt flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn gif1(&mut self) -> GIF1_W<0> {
        GIF1_W::new(self)
    }
    #[doc = "Bit 1 - transfer complete flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn tcif1(&mut self) -> TCIF1_W<1> {
        TCIF1_W::new(self)
    }
    #[doc = "Bit 2 - half transfer flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn htif1(&mut self) -> HTIF1_W<2> {
        HTIF1_W::new(self)
    }
    #[doc = "Bit 3 - transfer error flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn teif1(&mut self) -> TEIF1_W<3> {
        TEIF1_W::new(self)
    }
    #[doc = "Bit 4 - global interrupt flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn gif2(&mut self) -> GIF2_W<4> {
        GIF2_W::new(self)
    }
    #[doc = "Bit 5 - transfer complete flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn tcif2(&mut self) -> TCIF2_W<5> {
        TCIF2_W::new(self)
    }
    #[doc = "Bit 6 - half transfer flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn htif2(&mut self) -> HTIF2_W<6> {
        HTIF2_W::new(self)
    }
    #[doc = "Bit 7 - transfer error flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn teif2(&mut self) -> TEIF2_W<7> {
        TEIF2_W::new(self)
    }
    #[doc = "Bit 8 - global interrupt flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn gif3(&mut self) -> GIF3_W<8> {
        GIF3_W::new(self)
    }
    #[doc = "Bit 9 - transfer complete flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn tcif3(&mut self) -> TCIF3_W<9> {
        TCIF3_W::new(self)
    }
    #[doc = "Bit 10 - half transfer flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn htif3(&mut self) -> HTIF3_W<10> {
        HTIF3_W::new(self)
    }
    #[doc = "Bit 11 - transfer error flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn teif3(&mut self) -> TEIF3_W<11> {
        TEIF3_W::new(self)
    }
    #[doc = "Bit 12 - global interrupt flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn gif4(&mut self) -> GIF4_W<12> {
        GIF4_W::new(self)
    }
    #[doc = "Bit 13 - transfer complete flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn tcif4(&mut self) -> TCIF4_W<13> {
        TCIF4_W::new(self)
    }
    #[doc = "Bit 14 - half transfer flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn htif4(&mut self) -> HTIF4_W<14> {
        HTIF4_W::new(self)
    }
    #[doc = "Bit 15 - transfer error flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn teif4(&mut self) -> TEIF4_W<15> {
        TEIF4_W::new(self)
    }
    #[doc = "Bit 16 - global interrupt flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn gif5(&mut self) -> GIF5_W<16> {
        GIF5_W::new(self)
    }
    #[doc = "Bit 17 - transfer complete flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn tcif5(&mut self) -> TCIF5_W<17> {
        TCIF5_W::new(self)
    }
    #[doc = "Bit 18 - half transfer flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn htif5(&mut self) -> HTIF5_W<18> {
        HTIF5_W::new(self)
    }
    #[doc = "Bit 19 - transfer error flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn teif5(&mut self) -> TEIF5_W<19> {
        TEIF5_W::new(self)
    }
    #[doc = "Bit 20 - global interrupt flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn gif6(&mut self) -> GIF6_W<20> {
        GIF6_W::new(self)
    }
    #[doc = "Bit 21 - transfer complete flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn tcif6(&mut self) -> TCIF6_W<21> {
        TCIF6_W::new(self)
    }
    #[doc = "Bit 22 - half transfer flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn htif6(&mut self) -> HTIF6_W<22> {
        HTIF6_W::new(self)
    }
    #[doc = "Bit 23 - transfer error flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn teif6(&mut self) -> TEIF6_W<23> {
        TEIF6_W::new(self)
    }
    #[doc = "Bit 24 - global interrupt flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn gif7(&mut self) -> GIF7_W<24> {
        GIF7_W::new(self)
    }
    #[doc = "Bit 25 - transfer complete flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn tcif7(&mut self) -> TCIF7_W<25> {
        TCIF7_W::new(self)
    }
    #[doc = "Bit 26 - half transfer flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn htif7(&mut self) -> HTIF7_W<26> {
        HTIF7_W::new(self)
    }
    #[doc = "Bit 27 - transfer error flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn teif7(&mut self) -> TEIF7_W<27> {
        TEIF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `COUNT1_RX` reader"]
pub struct R(crate::R<COUNT1_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT1_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT1_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT1_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT1_RX` writer"]
pub struct W(crate::W<COUNT1_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT1_RX_SPEC>;
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
impl From<crate::W<COUNT1_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT1_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT1_RX` reader - Reception byte count"]
pub type COUNT1_RX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NUM_BLOCK` reader - Number of blocks"]
pub type NUM_BLOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_BLOCK` writer - Number of blocks"]
pub type NUM_BLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u16, COUNT1_RX_SPEC, u8, u8, 5, O>;
#[doc = "Field `BL_SIZE` reader - Block size"]
pub type BL_SIZE_R = crate::BitReader<bool>;
#[doc = "Field `BL_SIZE` writer - Block size"]
pub type BL_SIZE_W<'a, const O: u8> = crate::BitWriter<'a, u16, COUNT1_RX_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - Reception byte count"]
    #[inline(always)]
    pub fn count1_rx(&self) -> COUNT1_RX_R {
        COUNT1_RX_R::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 10:14 - Number of blocks"]
    #[inline(always)]
    pub fn num_block(&self) -> NUM_BLOCK_R {
        NUM_BLOCK_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Block size"]
    #[inline(always)]
    pub fn bl_size(&self) -> BL_SIZE_R {
        BL_SIZE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:14 - Number of blocks"]
    #[inline(always)]
    #[must_use]
    pub fn num_block(&mut self) -> NUM_BLOCK_W<10> {
        NUM_BLOCK_W::new(self)
    }
    #[doc = "Bit 15 - Block size"]
    #[inline(always)]
    #[must_use]
    pub fn bl_size(&mut self) -> BL_SIZE_W<15> {
        BL_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reception byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count1_rx](index.html) module"]
pub struct COUNT1_RX_SPEC;
impl crate::RegisterSpec for COUNT1_RX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [count1_rx::R](R) reader structure"]
impl crate::Readable for COUNT1_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count1_rx::W](W) writer structure"]
impl crate::Writable for COUNT1_RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT1_RX to value 0"]
impl crate::Resettable for COUNT1_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

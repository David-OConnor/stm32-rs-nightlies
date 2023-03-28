#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - AES enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - AES enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)"]
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)"]
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - AES operating mode"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - AES operating mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CHMOD` reader - AES chaining mode selection Bit1 Bit0"]
pub type CHMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHMOD` writer - AES chaining mode selection Bit1 Bit0"]
pub type CHMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CCFC` reader - Computation Complete Flag Clear"]
pub type CCFC_R = crate::BitReader<bool>;
#[doc = "Field `CCFC` writer - Computation Complete Flag Clear"]
pub type CCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ERRC` reader - Error clear"]
pub type ERRC_R = crate::BitReader<bool>;
#[doc = "Field `ERRC` writer - Error clear"]
pub type ERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CCFIE` reader - CCF flag interrupt enable"]
pub type CCFIE_R = crate::BitReader<bool>;
#[doc = "Field `CCFIE` writer - CCF flag interrupt enable"]
pub type CCFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAINEN` reader - Enable DMA management of data input phase"]
pub type DMAINEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAINEN` writer - Enable DMA management of data input phase"]
pub type DMAINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAOUTEN` reader - Enable DMA management of data output phase"]
pub type DMAOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAOUTEN` writer - Enable DMA management of data output phase"]
pub type DMAOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `GCMPH` reader - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
pub type GCMPH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GCMPH` writer - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
pub type GCMPH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CHMOD2` reader - AES chaining mode Bit2"]
pub type CHMOD2_R = crate::BitReader<bool>;
#[doc = "Field `CHMOD2` writer - AES chaining mode Bit2"]
pub type CHMOD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `KEYSIZE` reader - Key size selection"]
pub type KEYSIZE_R = crate::BitReader<bool>;
#[doc = "Field `KEYSIZE` writer - Key size selection"]
pub type KEYSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `NPBLB` reader - Number of padding bytes in last block of payload"]
pub type NPBLB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NPBLB` writer - Number of padding bytes in last block of payload"]
pub type NPBLB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - AES chaining mode selection Bit1 Bit0"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - AES chaining mode Bit2"]
    #[inline(always)]
    pub fn chmod2(&self) -> CHMOD2_R {
        CHMOD2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Key size selection"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block of payload"]
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<1> {
        DATATYPE_W::new(self)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    #[doc = "Bits 5:6 - AES chaining mode selection Bit1 Bit0"]
    #[inline(always)]
    #[must_use]
    pub fn chmod(&mut self) -> CHMOD_W<5> {
        CHMOD_W::new(self)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccfc(&mut self) -> CCFC_W<7> {
        CCFC_W::new(self)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ERRC_W<8> {
        ERRC_W::new(self)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccfie(&mut self) -> CCFIE_W<9> {
        CCFIE_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<10> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    #[must_use]
    pub fn dmainen(&mut self) -> DMAINEN_W<11> {
        DMAINEN_W::new(self)
    }
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    #[must_use]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<12> {
        DMAOUTEN_W::new(self)
    }
    #[doc = "Bits 13:14 - Used only for GCM, CCM and GMAC algorithms and has no effect when other algorithms are selected"]
    #[inline(always)]
    #[must_use]
    pub fn gcmph(&mut self) -> GCMPH_W<13> {
        GCMPH_W::new(self)
    }
    #[doc = "Bit 16 - AES chaining mode Bit2"]
    #[inline(always)]
    #[must_use]
    pub fn chmod2(&mut self) -> CHMOD2_W<16> {
        CHMOD2_W::new(self)
    }
    #[doc = "Bit 18 - Key size selection"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<18> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bits 20:23 - Number of padding bytes in last block of payload"]
    #[inline(always)]
    #[must_use]
    pub fn npblb(&mut self) -> NPBLB_W<20> {
        NPBLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
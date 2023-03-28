#[doc = "Register `BNDTR` reader"]
pub struct R(crate::R<BNDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BNDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BNDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BNDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BNDTR` writer"]
pub struct W(crate::W<BNDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BNDTR_SPEC>;
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
impl From<crate::W<BNDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BNDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNDT` reader - block number of data to transfer"]
pub type BNDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BNDT` writer - block number of data to transfer"]
pub type BNDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BNDTR_SPEC, u32, u32, 17, O>;
#[doc = "Field `BRSUM` reader - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BRSUM_R = crate::BitReader<bool>;
#[doc = "Field `BRSUM` writer - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BRSUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BNDTR_SPEC, bool, O>;
#[doc = "Field `BRDUM` reader - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BRDUM_R = crate::BitReader<bool>;
#[doc = "Field `BRDUM` writer - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BRDUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BNDTR_SPEC, bool, O>;
#[doc = "Field `BRC` reader - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
pub type BRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRC` writer - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
pub type BRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BNDTR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:16 - block number of data to transfer"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 18 - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - block number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<0> {
        BNDT_W::new(self)
    }
    #[doc = "Bit 18 - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn brsum(&mut self) -> BRSUM_W<18> {
        BRSUM_W::new(self)
    }
    #[doc = "Bit 19 - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn brdum(&mut self) -> BRDUM_W<19> {
        BRDUM_W::new(self)
    }
    #[doc = "Bits 20:31 - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn brc(&mut self) -> BRC_W<20> {
        BRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bndtr](index.html) module"]
pub struct BNDTR_SPEC;
impl crate::RegisterSpec for BNDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bndtr::R](R) reader structure"]
impl crate::Readable for BNDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bndtr::W](W) writer structure"]
impl crate::Writable for BNDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BNDTR to value 0"]
impl crate::Resettable for BNDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC0` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC0_R = crate::BitReader<bool>;
#[doc = "Field `SEC0` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC1` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC1_R = crate::BitReader<bool>;
#[doc = "Field `SEC1` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC2` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC2_R = crate::BitReader<bool>;
#[doc = "Field `SEC2` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC3` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC3_R = crate::BitReader<bool>;
#[doc = "Field `SEC3` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC4` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC4_R = crate::BitReader<bool>;
#[doc = "Field `SEC4` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC5` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC5_R = crate::BitReader<bool>;
#[doc = "Field `SEC5` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC6` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC6_R = crate::BitReader<bool>;
#[doc = "Field `SEC6` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC7` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC7_R = crate::BitReader<bool>;
#[doc = "Field `SEC7` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC8` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC8_R = crate::BitReader<bool>;
#[doc = "Field `SEC8` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC9` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC9_R = crate::BitReader<bool>;
#[doc = "Field `SEC9` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC10` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC10_R = crate::BitReader<bool>;
#[doc = "Field `SEC10` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC11` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC11_R = crate::BitReader<bool>;
#[doc = "Field `SEC11` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC12` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC12_R = crate::BitReader<bool>;
#[doc = "Field `SEC12` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC13` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC13_R = crate::BitReader<bool>;
#[doc = "Field `SEC13` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC14` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC14_R = crate::BitReader<bool>;
#[doc = "Field `SEC14` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC15` reader - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC15_R = crate::BitReader<bool>;
#[doc = "Field `SEC15` writer - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type SEC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec8(&self) -> SEC8_R {
        SEC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec9(&self) -> SEC9_R {
        SEC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec11(&self) -> SEC11_R {
        SEC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec12(&self) -> SEC12_R {
        SEC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec13(&self) -> SEC13_R {
        SEC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec14(&self) -> SEC14_R {
        SEC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn sec15(&self) -> SEC15_R {
        SEC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<0> {
        SEC0_W::new(self)
    }
    #[doc = "Bit 1 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<1> {
        SEC1_W::new(self)
    }
    #[doc = "Bit 2 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<2> {
        SEC2_W::new(self)
    }
    #[doc = "Bit 3 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<3> {
        SEC3_W::new(self)
    }
    #[doc = "Bit 4 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<4> {
        SEC4_W::new(self)
    }
    #[doc = "Bit 5 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<5> {
        SEC5_W::new(self)
    }
    #[doc = "Bit 6 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<6> {
        SEC6_W::new(self)
    }
    #[doc = "Bit 7 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<7> {
        SEC7_W::new(self)
    }
    #[doc = "Bit 8 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec8(&mut self) -> SEC8_W<8> {
        SEC8_W::new(self)
    }
    #[doc = "Bit 9 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec9(&mut self) -> SEC9_W<9> {
        SEC9_W::new(self)
    }
    #[doc = "Bit 10 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<10> {
        SEC10_W::new(self)
    }
    #[doc = "Bit 11 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec11(&mut self) -> SEC11_W<11> {
        SEC11_W::new(self)
    }
    #[doc = "Bit 12 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec12(&mut self) -> SEC12_W<12> {
        SEC12_W::new(self)
    }
    #[doc = "Bit 13 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec13(&mut self) -> SEC13_W<13> {
        SEC13_W::new(self)
    }
    #[doc = "Bit 14 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec14(&mut self) -> SEC14_W<14> {
        SEC14_W::new(self)
    }
    #[doc = "Bit 15 - I/O pin of Port x secure bit enable y (y = 15 to 0) These bits are written by software to enable or disable the I/O port pin security. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn sec15(&mut self) -> SEC15_W<15> {
        SEC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0xffff"]
impl crate::Resettable for SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}

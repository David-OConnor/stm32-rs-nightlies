#[doc = "Register `RCC_DDRITFCR` reader"]
pub struct R(crate::R<RCC_DDRITFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_DDRITFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_DDRITFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_DDRITFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_DDRITFCR` writer"]
pub struct W(crate::W<RCC_DDRITFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_DDRITFCR_SPEC>;
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
impl From<crate::W<RCC_DDRITFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_DDRITFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDRC1EN` reader - DDRC1EN"]
pub type DDRC1EN_R = crate::BitReader<bool>;
#[doc = "Field `DDRC1EN` writer - DDRC1EN"]
pub type DDRC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRC1LPEN` reader - DDRC1LPEN"]
pub type DDRC1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRC1LPEN` writer - DDRC1LPEN"]
pub type DDRC1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRC2EN` reader - DDRC2EN"]
pub type DDRC2EN_R = crate::BitReader<bool>;
#[doc = "Field `DDRC2EN` writer - DDRC2EN"]
pub type DDRC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRC2LPEN` reader - DDRC2LPEN"]
pub type DDRC2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRC2LPEN` writer - DDRC2LPEN"]
pub type DDRC2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRPHYCEN` reader - DDRPHYCEN"]
pub type DDRPHYCEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRPHYCEN` writer - DDRPHYCEN"]
pub type DDRPHYCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRPHYCLPEN` reader - DDRPHYCLPEN"]
pub type DDRPHYCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRPHYCLPEN` writer - DDRPHYCLPEN"]
pub type DDRPHYCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRCAPBEN` reader - DDRCAPBEN"]
pub type DDRCAPBEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRCAPBEN` writer - DDRCAPBEN"]
pub type DDRCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRCAPBLPEN` reader - DDRCAPBLPEN"]
pub type DDRCAPBLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRCAPBLPEN` writer - DDRCAPBLPEN"]
pub type DDRCAPBLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `AXIDCGEN` reader - AXIDCGEN"]
pub type AXIDCGEN_R = crate::BitReader<bool>;
#[doc = "Field `AXIDCGEN` writer - AXIDCGEN"]
pub type AXIDCGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRPHYCAPBEN` reader - DDRPHYCAPBEN"]
pub type DDRPHYCAPBEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRPHYCAPBEN` writer - DDRPHYCAPBEN"]
pub type DDRPHYCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRPHYCAPBLPEN` reader - DDRPHYCAPBLPEN"]
pub type DDRPHYCAPBLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRPHYCAPBLPEN` writer - DDRPHYCAPBLPEN"]
pub type DDRPHYCAPBLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `KERDCG_DLY` reader - KERDCG_DLY"]
pub type KERDCG_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KERDCG_DLY` writer - KERDCG_DLY"]
pub type KERDCG_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_DDRITFCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DDRCAPBRST` reader - DDRCAPBRST"]
pub type DDRCAPBRST_R = crate::BitReader<bool>;
#[doc = "Field `DDRCAPBRST` writer - DDRCAPBRST"]
pub type DDRCAPBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRCAXIRST` reader - DDRCAXIRST"]
pub type DDRCAXIRST_R = crate::BitReader<bool>;
#[doc = "Field `DDRCAXIRST` writer - DDRCAXIRST"]
pub type DDRCAXIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRCORERST` reader - DDRCORERST"]
pub type DDRCORERST_R = crate::BitReader<bool>;
#[doc = "Field `DDRCORERST` writer - DDRCORERST"]
pub type DDRCORERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DPHYAPBRST` reader - DPHYAPBRST"]
pub type DPHYAPBRST_R = crate::BitReader<bool>;
#[doc = "Field `DPHYAPBRST` writer - DPHYAPBRST"]
pub type DPHYAPBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DPHYRST` reader - DPHYRST"]
pub type DPHYRST_R = crate::BitReader<bool>;
#[doc = "Field `DPHYRST` writer - DPHYRST"]
pub type DPHYRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DPHYCTLRST` reader - DPHYCTLRST"]
pub type DPHYCTLRST_R = crate::BitReader<bool>;
#[doc = "Field `DPHYCTLRST` writer - DPHYCTLRST"]
pub type DPHYCTLRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DDRCKMOD` reader - DDRCKMOD"]
pub type DDRCKMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDRCKMOD` writer - DDRCKMOD"]
pub type DDRCKMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_DDRITFCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `GSKPMOD` reader - GSKPMOD"]
pub type GSKPMOD_R = crate::BitReader<bool>;
#[doc = "Field `GSKPMOD` writer - GSKPMOD"]
pub type GSKPMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `GSKPCTRL` reader - GSKPCTRL"]
pub type GSKPCTRL_R = crate::BitReader<bool>;
#[doc = "Field `GSKPCTRL` writer - GSKPCTRL"]
pub type GSKPCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DDRITFCR_SPEC, bool, O>;
#[doc = "Field `DFILP_WIDTH` reader - DFILP_WIDTH"]
pub type DFILP_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFILP_WIDTH` writer - DFILP_WIDTH"]
pub type DFILP_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_DDRITFCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `GSKP_DUR` reader - GSKP_DUR"]
pub type GSKP_DUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GSKP_DUR` writer - GSKP_DUR"]
pub type GSKP_DUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_DDRITFCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    pub fn ddrc1en(&self) -> DDRC1EN_R {
        DDRC1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    pub fn ddrc1lpen(&self) -> DDRC1LPEN_R {
        DDRC1LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    pub fn ddrc2en(&self) -> DDRC2EN_R {
        DDRC2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    pub fn ddrc2lpen(&self) -> DDRC2LPEN_R {
        DDRC2LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    pub fn ddrphycen(&self) -> DDRPHYCEN_R {
        DDRPHYCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    pub fn ddrphyclpen(&self) -> DDRPHYCLPEN_R {
        DDRPHYCLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    pub fn ddrcapben(&self) -> DDRCAPBEN_R {
        DDRCAPBEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    pub fn ddrcapblpen(&self) -> DDRCAPBLPEN_R {
        DDRCAPBLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    pub fn axidcgen(&self) -> AXIDCGEN_R {
        AXIDCGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    pub fn ddrphycapben(&self) -> DDRPHYCAPBEN_R {
        DDRPHYCAPBEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    pub fn ddrphycapblpen(&self) -> DDRPHYCAPBLPEN_R {
        DDRPHYCAPBLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    pub fn kerdcg_dly(&self) -> KERDCG_DLY_R {
        KERDCG_DLY_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    pub fn ddrcapbrst(&self) -> DDRCAPBRST_R {
        DDRCAPBRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    pub fn ddrcaxirst(&self) -> DDRCAXIRST_R {
        DDRCAXIRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    pub fn ddrcorerst(&self) -> DDRCORERST_R {
        DDRCORERST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    pub fn dphyapbrst(&self) -> DPHYAPBRST_R {
        DPHYAPBRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    pub fn dphyrst(&self) -> DPHYRST_R {
        DPHYRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    pub fn dphyctlrst(&self) -> DPHYCTLRST_R {
        DPHYCTLRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    pub fn ddrckmod(&self) -> DDRCKMOD_R {
        DDRCKMOD_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    pub fn gskpmod(&self) -> GSKPMOD_R {
        GSKPMOD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    pub fn gskpctrl(&self) -> GSKPCTRL_R {
        GSKPCTRL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    pub fn dfilp_width(&self) -> DFILP_WIDTH_R {
        DFILP_WIDTH_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    pub fn gskp_dur(&self) -> GSKP_DUR_R {
        GSKP_DUR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc1en(&mut self) -> DDRC1EN_W<0> {
        DDRC1EN_W::new(self)
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc1lpen(&mut self) -> DDRC1LPEN_W<1> {
        DDRC1LPEN_W::new(self)
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc2en(&mut self) -> DDRC2EN_W<2> {
        DDRC2EN_W::new(self)
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrc2lpen(&mut self) -> DDRC2LPEN_W<3> {
        DDRC2LPEN_W::new(self)
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrphycen(&mut self) -> DDRPHYCEN_W<4> {
        DDRPHYCEN_W::new(self)
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrphyclpen(&mut self) -> DDRPHYCLPEN_W<5> {
        DDRPHYCLPEN_W::new(self)
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcapben(&mut self) -> DDRCAPBEN_W<6> {
        DDRCAPBEN_W::new(self)
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcapblpen(&mut self) -> DDRCAPBLPEN_W<7> {
        DDRCAPBLPEN_W::new(self)
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    #[must_use]
    pub fn axidcgen(&mut self) -> AXIDCGEN_W<8> {
        AXIDCGEN_W::new(self)
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrphycapben(&mut self) -> DDRPHYCAPBEN_W<9> {
        DDRPHYCAPBEN_W::new(self)
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrphycapblpen(&mut self) -> DDRPHYCAPBLPEN_W<10> {
        DDRPHYCAPBLPEN_W::new(self)
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    #[must_use]
    pub fn kerdcg_dly(&mut self) -> KERDCG_DLY_W<11> {
        KERDCG_DLY_W::new(self)
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcapbrst(&mut self) -> DDRCAPBRST_W<14> {
        DDRCAPBRST_W::new(self)
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcaxirst(&mut self) -> DDRCAXIRST_W<15> {
        DDRCAXIRST_W::new(self)
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    #[must_use]
    pub fn ddrcorerst(&mut self) -> DDRCORERST_W<16> {
        DDRCORERST_W::new(self)
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    #[must_use]
    pub fn dphyapbrst(&mut self) -> DPHYAPBRST_W<17> {
        DPHYAPBRST_W::new(self)
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    #[must_use]
    pub fn dphyrst(&mut self) -> DPHYRST_W<18> {
        DPHYRST_W::new(self)
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    #[must_use]
    pub fn dphyctlrst(&mut self) -> DPHYCTLRST_W<19> {
        DPHYCTLRST_W::new(self)
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    #[must_use]
    pub fn ddrckmod(&mut self) -> DDRCKMOD_W<20> {
        DDRCKMOD_W::new(self)
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    #[must_use]
    pub fn gskpmod(&mut self) -> GSKPMOD_W<23> {
        GSKPMOD_W::new(self)
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn gskpctrl(&mut self) -> GSKPCTRL_W<24> {
        GSKPCTRL_W::new(self)
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn dfilp_width(&mut self) -> DFILP_WIDTH_W<25> {
        DFILP_WIDTH_W::new(self)
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    #[must_use]
    pub fn gskp_dur(&mut self) -> GSKP_DUR_W<28> {
        GSKP_DUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ddritfcr](index.html) module"]
pub struct RCC_DDRITFCR_SPEC;
impl crate::RegisterSpec for RCC_DDRITFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ddritfcr::R](R) reader structure"]
impl crate::Readable for RCC_DDRITFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ddritfcr::W](W) writer structure"]
impl crate::Writable for RCC_DDRITFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_DDRITFCR to value 0x000f_d02a"]
impl crate::Resettable for RCC_DDRITFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_d02a;
}
